use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

// 親スレッドから子スレッド2へのメッセージ
pub enum ToChild2Message {
    Run,
    Exit,
}

// 子スレッド2から親スレッドへのメッセージ
pub enum FromChild2Message {
    Done,
}

// 子スレッド2の実装(独立したライブラリに実装という設定)。親スレッドと異なるcrateのためToParentMessage を使えない。
// そこでジェネリックな関数を使って親スレッドへのメッセージ送信を実現する。
// ジェネリックな関数の中身に関しては使用側(親スレッド)に任せる。
pub fn child2_thread<F>(
    to_parent_sender_func: &F,
    to_child2_receiver: Receiver<ToChild2Message>,
)
where F: Fn(FromChild2Message),
{
    loop{
        match to_child2_receiver.recv().unwrap() {
            ToChild2Message::Run => {
                // 適当な処理(3までカウントアップする)
                for i in 0..4 {
                    println!("Child2: count = {}",i);
                    thread::sleep(Duration::new(1,0));
                }
                // 親スレッドへ処理完了メッセージを送る
                to_parent_sender_func(FromChild2Message::Done);
            }
            ToChild2Message::Exit => {
                // スレッドを終了させる
                break;
            }
        }
    }
}