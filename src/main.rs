use html_markdown_converter;

fn main(){
    let html = r#"
        <div>Regular div</div>
        <div src="https://example.com"></div>
    "#;
    let result = html_markdown_converter::html_to_markdown(html);
    println!("{}", result);
}