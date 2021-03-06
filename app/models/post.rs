use db;

pub struct Post {
    pub id: i64,
    pub author: String,
    pub date: String,
    pub title: String,
    pub content: String
}

impl Post {
    pub fn find(id: i64) -> Post {
        let conn = db::connection();

        conn.query_row("SELECT * FROM post WHERE id=$1", &[&id], |row| {
            Post {
                id: row.get(0),
                author: row.get(1),
                date: row.get(2),
                content: row.get(3),
                title: row.get(4)
            }
        }).unwrap()
    }

    pub fn all() -> Vec<Post> {
        let conn = db::connection();

        let mut posts = Vec::new();
        let mut stmt = conn.prepare("SELECT * FROM post").unwrap();
        let rows = stmt.query(&[]).unwrap();

        for row in rows {
            let row = row.unwrap();
            posts.push(
                Post {
                    id: row.get(0),
                    author: row.get(1),
                    date: row.get(2),
                    content: row.get(3),
                    title: row.get(4)
                }
            );
        }

        posts
    }
}
