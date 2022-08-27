use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

// 親スレッドへのメッセージを使う
use super::ToParentMessage;

// 親スレッドから子スレッド1へのメッセージ
pub enum ToChild1Message {
    Run,
    Exit,
}

// 子スレッド1から親スレッドへのメッセージ
pub enum FromChild1Message {
    Done,
}

// 子スレッド1の実装。親スレッドと同じcrate内にあるため、ToParentMessage を使える。
// 異なるcrateに存在する場合の解決方法は sample_lib の子スレッド2の実装を参照。
pub fn child1_thread(
    to_parent_sender: Sender<ToParentMessage>,
    to_child1_receiver: Receiver<ToChild1Message>,
){
    loop{
        match to_child1_receiver.recv().unwrap() {
            ToChild1Message::Run => {
                // 適当な処理(3までカウントアップする)
                for i in 0..4 {
                    println!("Child1: count = {}",i);
                    thread::sleep(Duration::new(1,0));
                }
                // 親スレッドへ処理完了メッセージを送る
                to_parent_sender.send(ToParentMessage::Child1(FromChild1Message::Done)).unwrap();
            }
            ToChild1Message::Exit => {
                // スレッドを終了させる
                break;
            }
        }
    }
}