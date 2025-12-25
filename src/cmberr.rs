
pub trait CombineError<EA, EB> {
    type Out;

    fn left(&mut self, ea: EA);
    fn right(&mut self, eb: EB);
    fn finish(self) -> Self::Out;
}

pub trait CombineErrorBuilder<EA, EB> {
    type Combiner: CombineError<EA, EB, Out = Self::Out>;
    type Out;

    fn build() -> Self::Combiner;
}

// ======================= DefaultCombine =======================

pub struct DefaultCombine<E> {
    data: Option<E>
}

impl<E> CombineErrorBuilder<E, E> for DefaultCombine<E> {
    type Combiner = Self;
    type Out = E;

    fn build() -> Self::Combiner {
        DefaultCombine { data:None }
    }
}

impl<E> CombineError<E, E> for DefaultCombine<E> {
    type Out = E;

    fn left(&mut self, ea: E) {
        self.data = Some(ea);
    }

    fn right(&mut self, eb: E) {
        self.data = Some(eb);
    }

    fn finish(self) -> Self::Out {
        self.data.unwrap()
    }
}

// ======================= CustomCombine =======================

pub struct VecCombine<T> {
    data: Vec<T>
}

impl<E> CombineErrorBuilder<E, E> for VecCombine<E> {
    type Combiner = Self;
    type Out = Vec<E>;

    fn build() -> Self::Combiner {
        VecCombine { data: Vec::new() }
    }
}

impl<E> CombineErrorBuilder<Vec<E>, E> for VecCombine<E> {
    type Combiner = Self;
    type Out = Vec<E>;

    fn build() -> Self::Combiner {
        VecCombine { data: Vec::new() }
    }
}

impl<T> CombineError<T, T> for VecCombine<T> {
    type Out = Vec<T>;

    fn left(&mut self, ea: T) {
        self.data.push(ea);
    }

    fn right(&mut self, eb: T) {
        self.data.push(eb);
    }

    fn finish(self) -> Self::Out {
        self.data
    }
}

impl<E> CombineError<Vec<E>, E> for VecCombine<E> {
    type Out = Vec<E>;

    fn left(&mut self, ea: Vec<E>) {
        self.data.extend(ea);
    }

    fn right(&mut self, eb: E) {
        self.data.push(eb);
    }

    fn finish(self) -> Self::Out {
        self.data
    }
}

