fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    if SomeTaskDone(cx) {
        return Poll::Ready(TheReturnValue);
    }
    return Poll::Pending;
}

enum StateMachine<T> {
    NotReady(Context),
    Ready(T),
    Done,
}

fn main() {
    let mut state_machine = StateMachine::NotReady(cx);
    loop {
        match state_machine {
            StateMachine::NotReady(cx) => {
                if SomeTaskDone(cx) {
                    state_machine = StateMachine::Ready(TheReturnValue);
                } else {
                    state_machine = StateMachine::NotReady(cx);
                }
            }
            StateMachine::Ready(value) => {
                return Poll::Ready(value);
            }
            StateMachine::Done => {
                return Poll::Ready(None);
            }
        }
    }
}
