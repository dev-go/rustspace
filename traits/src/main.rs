use traits::Summary;

fn main() {
    let tweet = traits::Tweet::create(
        String::from("TWEET.username"),
        String::from("TWEET.content"),
        false,
        false,
    );
    println!("tweet: {:?}", tweet);
    println!("==> summarize: {}", tweet.summarize());
    println!("==> summarize: {}", traits::Summary::summarize(&tweet));

    let news = traits::News::create(
        String::from("NEWS.headline"),
        String::from("NEWS.content"),
        String::from("NEWS.author"),
        String::from("NEWS.location"),
    );
    println!("news: {:?}", news);
    println!("==> summarize: {}", news.summarize());
    println!("==> summarize: {}", traits::Summary::summarize(&news));

    let mut summary: &dyn traits::Summary = &news;
    println!("summary: {}", summary.summarize());
    summary = &tweet;
    println!("summary: {}", summary.summarize());
    println!("summary_default: {}", summary.summarize_default());

    traits::notify(&tweet);
    traits::notify(&news);
}
