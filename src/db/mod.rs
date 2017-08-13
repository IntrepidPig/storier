pub mod config;

use postgres::{Connection, TlsMode};
use blog::{Post, Author};

pub struct DB {
    conn: Connection,
}

unsafe impl Sync for DB {}

impl DB {
    pub fn new(conn_str: String) -> DB {
        DB {
            conn: Connection::connect(conn_str, TlsMode::None).unwrap()
        }
    }

    pub fn get_posts(&self, from: u32, amount: u32) -> Vec<Post> {
        let mut posts: Vec<Post> = Vec::new();

        for (i, row) in self.conn.query("SELECT * FROM posts ORDER BY date DESC", &[]).unwrap().iter().enumerate() {
            if i >= from as usize && i <= (from + amount) as usize {
                posts.push(Post {
                    id: row.get(0),
                    safe_title: row.get(1),
                    raw_title: row.get(2),
                    date: row.get(3),
                    text: row.get(4),
                    author: self.find_author_by_id(row.get(5)),
                });
            }
        }

        posts
    }

    pub fn get_post_by_title(&self, title: String) -> Option<Post> {
        for row in self.conn.query("SELECT * FROM posts WHERE safe_title = $1", &[&title]).unwrap().iter() {
            return Some(Post {
                id: row.get(0),
                safe_title: row.get(1),
                raw_title: row.get(2),
                date: row.get(3),
                text: row.get(4),
                author: self.find_author_by_id(row.get(5)),
            });
        }

        None
    }

    pub fn add_post(&self, post: Post) {
        self.conn.execute("INSERT INTO posts (safe_title, raw_title, date, content, author)\
                           VALUES ($1, $2, $3, $4, $5)", &[&post.safe_title, &post.raw_title, &post.date, &post.text, &4]).unwrap();
    }

    #[allow(unused_variables)]
    pub fn find_author_by_id(&self, id: i32) -> Author {
        Author { name: String::from("IntrepidPig") }
    }

    #[allow(unused_variables)]
    pub fn check_safe_title(&self, title: String) {
        // TODO
    }
}