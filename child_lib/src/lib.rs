use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// 親スレッドから子スレッドへのメッセージ
pub enum ToChildMessage {
    Run,
    Exit,
}

// 子スレッドから親スレッドへのメッセージ
pub enum FromChildMessage {
    Done,
}

// 子スレッドのメッセージ受信ループ用。
// 列挙子は現実装では一つだが、この子スレッドがさらに子スレッド（=孫スレッド）を持つ場合にはFromGrandChild(FromGrandChild)みたいなものが追加されるイメージ。
enum ToChildInnerMessage {
    FromParent(ToChildMessage)
}

// 子スレッド
fn child_thread<F>(
    to_parent_sender_func: F,
    to_child_receiver: Receiver<ToChildInnerMessage>,
)
where F: Fn(FromChildMessage),
{
    loop{
        match to_child_receiver.recv().unwrap() {
            ToChildInnerMessage::FromParent(message) => match message {
                ToChildMessage::Run => {
                    // 適当な処理(3までカウントアップする)
                    for i in 0..4 {
                        println!("Child: count = {}",i);
                        thread::sleep(Duration::new(1,0));
                    }
                    // 親スレッドへ処理完了メッセージを送る
                    to_parent_sender_func(FromChildMessage::Done);
                }
                ToChildMessage::Exit => {
                    // スレッドを終了させる
                    break;
                }
            }
            // 孫スレッドが存在する場合などは以下に続く
        }
    }
}


// ジェネリックな関数を使って親スレッドへのメッセージ送信を実現する。
// ジェネリックな関数の中身に関しては使用側(親スレッド)に任せる。
pub struct ChildThread{
    to_child_sender: Sender<ToChildInnerMessage>,
    op_child_thread: Option<JoinHandle<()>>,
}

impl ChildThread {
    pub fn new<F>(
        to_parent_sender_func: F,
    ) -> ChildThread 
    where F: Fn(FromChildMessage) + std::marker::Send + 'static,
    {
        let (to_child_sender, to_child_receiver) = channel::<ToChildInnerMessage>();
        let child_thread = thread::spawn(move || child_thread(
            to_parent_sender_func, 
            to_child_receiver));
        ChildThread {
            to_child_sender: to_child_sender,
            op_child_thread:Some(child_thread),
        }
    }

    // 親スレッドから子スレッド向けにメッセージを送る。ToChildInnerMessage::FromParent()でくるんでsend()する。
    pub fn send(&self, message: ToChildMessage) {
        self.to_child_sender.send(ToChildInnerMessage::FromParent(message)).unwrap()
    }

    // この実装ではマニュアルでExit⇒join()するようにさせているが、impl Drop fpr ChildThreadも実装すると良いと思う（Exitメッセージのsend()も込みで。）。
    pub fn join(&mut self) {
        let op_child_thread = self.op_child_thread.take();
        if let Some(child_thread) = op_child_thread {
            child_thread.join().unwrap()
        }
    }
}
