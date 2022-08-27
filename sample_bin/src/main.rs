use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

extern crate child2_lib;
use child2_lib::*;

mod child1;
use child1::*;

// 各子スレッドから親スレッドへのメッセージ
// 一か所でReceiveするためそれぞれの子スレッドからのメッセージ型を保有する。
pub enum ToParentMessage {
    Child1(FromChild1Message),
    Child2(FromChild2Message),
}

// main関数(親スレッド)
fn main() {
    // 子スレッドから親スレッドへの Sender、 Reciever 。
    let (to_parent_sender, to_parent_receiver) = channel::<ToParentMessage>();
    // 子スレッド作成時に所有権が移動するので Sender をクローンしておく。
    let to_parent_sender_clone1 = to_parent_sender.clone();
    let to_parent_sender_clone2 = to_parent_sender.clone();

    // 親スレッドから子スレッド1への Sender、 Reciever 。
    let (to_child1_sender, to_child1_receiver) = channel::<ToChild1Message>();
    // 子スレッド1作成。child1_threadへの引数は、親スレッドへの Sender と子スレッド1への Reciever。
    let child1_thread_instance = thread::spawn(move || child1_thread(
        to_parent_sender_clone1,
        to_child1_receiver));

    // 親スレッドから子スレッド2への Sender、 Reciever 。
    let (to_child2_sender, to_child2_receiver) = channel::<ToChild2Message>();
    // 子スレッド2作成。child2_threadへの引数は、「親スレッドへの Sender によるsend()を実装したクロージャ」と子スレッド1への Reciever。
    let child2_thread_instance = thread::spawn(move || child2_thread(
        &|x| to_parent_sender_clone2.send(ToParentMessage::Child2(x)).unwrap(), 
        to_child2_receiver));

    // ここの処理はサンプル用に子スレッドにメッセージを送信しているだけ
    to_child1_sender.send(ToChild1Message::Run).unwrap();
    thread::sleep(Duration::new(0, 500000000));
    to_child2_sender.send(ToChild2Message::Run).unwrap();

    // loopを抜けるための雑な仕組み
    let mut sent_exit1 = false;
    let mut sent_exit2 = false;

    // 親スレッドの Receiver を使ったイベントドリブンなループ
    loop {
        match to_parent_receiver.recv().unwrap() {
            // 子スレッド１からのイベント
            ToParentMessage::Child1(message) => {
                match message {
                    FromChild1Message::Done => {
                        println!("Child1: Return \"Done\".");
                        to_child1_sender.send(ToChild1Message::Exit).unwrap();
                        sent_exit1 = true;
                        if sent_exit2 {
                            break;
                        }
                    }
                    // 実際はもっといろいろなメッセージがあるはず。
                }
            }
            // 子スレッド2からのイベント
            ToParentMessage::Child2(message) => {
                match message {
                    FromChild2Message::Done => {
                        println!("Child2: Return \"Done\".");
                        to_child2_sender.send(ToChild2Message::Exit).unwrap();
                        sent_exit2 = true;
                        if sent_exit1 {
                            break;
                        }
                    }
                    // 実際はもっといろいろなメッセージがあるはず。
                }
            }
        }
    }
    child1_thread_instance.join().unwrap();
    child2_thread_instance.join().unwrap();
    println!("All child threads joined.");
}
