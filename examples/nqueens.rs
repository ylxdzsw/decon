use std::rc::Rc;

use decon::*;

type Board = Vec<i32>;

#[derive(Clone)]
struct Rec<T>(Rc<ContBoxClonable<(Rec<T>, T)>>);

fn main() {
    nqueens(8)
}

#[reset_func]
fn nqueens(n: i32) {
    // board[i] = j means a queen in (i,j)
    let (loop_back, mut board): (Rec<Board>, Board) = shift(get_cc);
    let next_row = board.len();
    let next = shift(fork(0..n));

    if board.iter().enumerate().any(|(i, j)| j == &next || (next - j).abs() as usize == next_row - i) {
        return
    }

    board.push(next);
    if board.len() < n as _ {
        loop_back.0((loop_back.clone(), board));
    } else {
        println!("{:?}", board);
    }
}

fn get_cc<T: Default>(cont: ContBoxClonable<(Rec<T>, T)>) {
    let cont = Rc::new(cont);
    cont((Rec(cont.clone()), Default::default()))
}

fn fork<T: Clone>(iter: impl IntoIterator<Item=T>) -> impl FnOnce(ContBoxOnceClonable<T>) {
    move |cont| {
        for i in iter {
            cont.clone()(i);
        }
    }
}

// TODO: simplify the loop_back by store it to an Rc<RefCell<>> in get_cc, so its signature can be simply ContBoxClonable<T, ()>