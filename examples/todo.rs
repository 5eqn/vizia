use vizia::*;

fn main() {
    Application::new(|cx|{

        TodoData {
            items: vec![
                    TodoItem {
                        text: "Item 1".to_string(),
                        completed: false,
                    },

                    TodoItem {
                        text: "Item 2".to_string(),
                        completed: true,
                    }
                ],
            selected: 0,
        }.build(cx);

        List::new(cx, TodoData::items, |cx, index, item|{
            
            //let item = item.clone();
            Binding::new(cx, TodoData::selected, move |cx, selected|{
                HStack::new(cx, move |cx|{
                    let selected = *selected.get(cx);
                    Label::new(cx, &item.text.to_owned()).background_color(if selected == index {
                        Color::green()
                    } else {
                        Color::blue()
                    });
                    Label::new(cx, &item.completed.to_string());
                });
            });
        });
    
        // Binding::new(TodoData::items).build(cx, |cx, items|{
        //     HStack::new().build(cx, |cx|{
        //         Button::new(|cx| items.get(cx))
        //     });      
        // });
    }).run();
}

#[derive(Lens, Clone)]
pub struct TodoItem {
    text: String,
    completed: bool,
}

#[derive(Lens)]
pub struct TodoData {
    items: Vec<TodoItem>,
    selected: usize,
}

impl Model for TodoData {

}