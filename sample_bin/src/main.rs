use std::sync::mpsc::channel;

extern crate child_lib;
use child_lib::*;

// 各子スレッドから親スレッドへのメッセージ
// 一か所でReceiveするためそれぞれの子スレッドからのメッセージ型を保有する。
// この実装では列挙子は1つだけだが、Child2(FromChild2Message)などが追加されるイメージ。
pub enum ToParentMessage {
    Child(FromChildMessage),
}

// main関数(親スレッド)
fn main() {
    // 子スレッドから親スレッドへの Sender、 Reciever 。
    let (to_parent_sender, to_parent_receiver) = channel::<ToParentMessage>();
    // 子スレッド作成時に所有権が移動するので Sender をクローンしておく。（今回は子スレッドが1つしかないので本当は必要ないはず）
    let to_parent_sender_clone = to_parent_sender.clone();

    // 子スレッドのインスタンス作成。引数は「親スレッドへの Sender によるsend()を実装したクロージャ」。
    let mut child = ChildThread::new(
        move |x| to_parent_sender_clone.send(ToParentMessage::Child(x)).unwrap()
    );

    // 子スレッドにメッセージを送るときはsend()メソッドで送る。
    child.send(ToChildMessage::Run);

    // 親スレッドの Receiver を使ったイベントドリブンなループ
    loop {
        match to_parent_receiver.recv().unwrap() {
            // 子スレッドからのイベント
            ToParentMessage::Child(message) => {
                match message {
                    FromChildMessage::Done => {
                        println!("Child: Return \"Done\".");
                        break;
                    }
                    // 実際はもっといろいろなメッセージがあるはず。
                }
            }
            // 他の子スレッドがある場合は以下に続く
        }
    }
    // 子スレッド終了処理。Dropに実装しておいてもよい。
    child.send(ToChildMessage::Exit);
    child.join();
    println!("the child thread joined.");
}
