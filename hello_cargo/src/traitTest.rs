pub trait Summary {
    fn summary(&self) -> String;

    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub trait Content {
    fn showContents(&self) -> String;
}

pub struct NewArticle {
    pub title: String,
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
} 

impl Summary for NewArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Content for NewArticle {
    fn showContents(&self) -> String {
        format!("{}", self.content)
    }
}



pub struct Tweet {
    pub userName: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.userName, self.content)
    }
}

impl Content for Tweet {
    fn showContents(&self) -> String {
        format!("{}", self.content)
    }
}



fn notify(item: &impl Summary) {
   println!("notify : {}",  item.summarize());
}

// 两个参数只要实现了Summary就行
fn notify1(item1: &impl Summary, item2: &impl Summary) {
    println!("item1 : {}, item2 : {}",  item1.summarize(), item2.summarize());
}

// 指明两个参数item1和item2是相同类型，就算实现了同一个trait，只要是不同类型就不行
fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("item1 : {}, item2 : {}",  item1.summarize(), item2.summarize());
}

fn notify3<T: Summary + Content>(item1: &T, item2: &T) {
    println!("item1.summarize : {}, item1.content : {}\nitem2.content : {}, item2.summarize : {}",  item1.summarize(), item1.showContents(), item2.summarize(), item2.showContents());
}

fn notify4<T, U>(item1: &T, item2: &U) 
where T: Summary + Content,
      U: Summary + Content
{
    println!("item1.summarize : {}, item1.content : {}\nitem2.content : {}, item2.summarize : {}",  item1.summarize(), item1.showContents(), item2.summarize(), item2.showContents());
}

fn main() {
    let article = NewArticle {
        title: String::from("test"), 
        headline: String::from("测试"), 
        author: String::from("yinghao"), 
        location: String::from("hangzhou"), 
        content: String::from("测试")
    };
    println!("article summary : {}", article.summary());
    println!("article summarize : {}", article.summarize());

    let tweet = Tweet {
        userName: String::from("yinghao"), 
        content: String::from("内容"), 
        reply: false, retweet: false
    };
    println!("tweet summary : {}", tweet.summary());
    println!("tweet summarize : {}", article.summarize());

    println!("--------------");
    notify(&article);
    notify(&tweet);
    
    println!("-------1-------");
    notify1(&article, &tweet);
    
    println!("-------2-------");
    notify2(&article, &article);
    
    println!("-------3-------");
    notify3(&article, &article);
    
    println!("-------4-------");
    notify4(&article, &article);
}