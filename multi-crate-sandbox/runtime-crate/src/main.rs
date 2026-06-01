use lib_crate::{self, ImportantExcerpt};

fn main() {
    let excerpt = ImportantExcerpt{ title: "title1", content: "content1" };
    excerpt.announce_and_return_part("announcement");
}
