struct Tweet <'a>{
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str{
        let oc = self.content;
        self.content = content;
        oc
    }
}

fn main() {
    let mut t = Tweet{
        content: "hello",
    };
    let oc = t.replace_content("world");
}

fn take_return_content(content: &str) -> &str {
    content
}