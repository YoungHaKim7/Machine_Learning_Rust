use rust_bert::pipelines::question_answering::QuestionAnsweringModel;
use rust_bert::pipelines::question_answering::QaInput;

fn main() {
    let qa_model = QuestionAnsweringModel::new(Default::default());
    let question = String::from("Where does Amy live");
    let context = String::from("Amy lives in Amsterdam");

    let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);
}
