
mod infer;
mod prolog;

use crate::infer::{InferenceEngine, KnowledgeBase};
use crate::prolog::{u, v, a, PrologInferenceEngine, PrologSyntax, f, p, t};

impl KnowledgeBase<PrologSyntax> for Vec<PrologSyntax> {
    fn contains(&self, l: PrologSyntax) -> bool {
        self.contains(l)
    }
}

fn main() {
    let knowledge = vec![];
    PrologInferenceEngine::infer(&knowledge, u(v("A"), a("a")));
    PrologInferenceEngine::infer(&knowledge, u(p("is", &[ v("A"), a("b") ]), p("is", &[ a("a"), v("B") ])));
    PrologInferenceEngine::infer(&knowledge, u(t(), p("foo", &[ a("A") ])));
}