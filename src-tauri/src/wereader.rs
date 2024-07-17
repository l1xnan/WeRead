// https://github.com/arry-lee/wereader
use reqwest::{Client, Error, Response};
use std::collections::HashMap;
use std::default::Default;

#[derive(Debug)]
struct Book {
  book_id: String,
  title: String,
  author: String,
  cover: String,
}

// 获取某本书的笔记，返回 markdown 文本
fn get_bookmarklist(book_id: &str, cookies: &HashMap<String, String>) -> String {
  let url = "https://i.weread.qq.com/book/bookmarklist";
  let mut params = HashMap::new();
  params.insert("bookId", book_id);

  let client = Client::new();
  let res = client
    .get(url)
    .query(&params)
    .headers(get_headers())
    .cookies(cookies)
    .send()
    .unwrap_or_else(|e| panic!("Request error: {}", e));

  if res.status().is_success() {
    let data: serde_json::Value = res
      .json()
      .unwrap_or_else(|e| panic!("JSON parsing error: {}", e));
    let mut chapters = HashMap::new();
    for c in data["chapters"].as_array().unwrap() {
      let chapter_uid = c["chapterUid"].as_str().unwrap();
      let title = c["title"].as_str().unwrap();
      chapters.insert(chapter_uid.to_string(), title.to_string());
    }

    let mut contents = HashMap::default();
    for item in data["updated"]
      .as_array()
      .unwrap()
      .iter()
      .sorted_by_key(|x| x["chapterUid"].as_str().unwrap())
    {
      let chapter = item["chapterUid"].as_str().unwrap();
      let text = item["markText"].as_str().unwrap();
      let create_time = item["createTime"].as_str().unwrap();
      let start = item["range"]
        .split("-")
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap();
      contents
        .entry(chapter.to_string())
        .or_default()
        .push((start, text.to_string()));
    }

    let chapters_map = get_chapters(book_id, cookies);
    let mut res = String::new();
    for c in chapters.keys() {
      let title = chapters.get(c).unwrap();
      res += &("#".repeat(chapters_map[title]) + " " + title + "\n");
      for (start, text) in contents.get(c).unwrap().iter().sorted_by_key(|e| e.0) {
        res += &("> " + &text.strip() + "\n\n");
      }
      res += "\n";
    }
    res
  } else {
    panic!("Request failed with status: {}", res.status());
  }
}

// 获取书籍的热门划线,返回文本
fn get_bestbookmarks(book_id: &str, cookies: &HashMap<String, String>) -> String {
  let url = "https://i.weread.qq.com/book/bestbookmarks";
  let mut params = HashMap::new();
  params.insert("bookId", book_id);

  let client = Client::new();
  let res = client
    .get(url)
    .query(&params)
    .headers(get_headers())
    .cookies(cookies)
    .send()
    .unwrap_or_else(|e| panic!("Request error: {}", e));

  if res.status().is_success() {
    let data: serde_json::Value = res
      .json()
      .unwrap_or_else(|e| panic!("JSON parsing error: {}", e));
    let mut chapters = HashMap::new();
    for c in data["chapters"].as_array().unwrap() {
      let chapter_uid = c["chapterUid"].as_str().unwrap();
      let title = c["title"].as_str().unwrap();
      chapters.insert(chapter_uid.to_string(), title.to_string());
    }

    let mut contents = HashMap::default();
    for item in data["items"].as_array().unwrap() {
      let chapter = item["chapterUid"].as_str().unwrap();
      let text = item["markText"].as_str().unwrap();
      contents
        .entry(chapter.to_string())
        .or_default()
        .push(text.to_string());
    }

    let chapters_map = get_chapters(book_id, cookies);
    let mut res = String::new();
    for c in chapters.keys() {
      let title = chapters.get(c).unwrap();
      res += &("#".repeat(chapters_map[title]) + " " + title + "\n");
      for text in contents.get(c).unwrap() {
        res += &("> " + &text.strip() + "\n\n");
      }
      res += "\n";
    }
    res
  } else {
    panic!("Request failed with status: {}", res.status());
  }
}

// 获取书的目录
fn get_chapters(book_id: &str, cookies: &HashMap<String, String>) -> HashMap<String, usize> {
  let url = "https://i.weread.qq.com/book/chapterInfos";
  let data = format!("{{\"bookIds\":[\"{}\"],\"synckeys\":[0]}}", book_id);

  let client = Client::new();
  let res = client
    .post(url)
    .body(data)
    .headers(get_headers())
    .cookies(cookies)
    .send()
    .unwrap_or_else(|e| panic!("Request error: {}", e));

  if res.status().is_success() {
    let data: serde_json::Value = res
      .json()
      .unwrap_or_else(|e| panic!("JSON parsing error: {}", e));
    let mut chapters = HashMap::new();
    for item in data["data"][0]["updated"].as_array().unwrap() {
      if item.contains_key("anchors") {
        let level = item
          .get("level")
          .unwrap_or(&serde_json::Value::from(1))
          .as_u64()
          .unwrap() as usize;
        let title = item["title"].as_str().unwrap();
        chapters.insert(title.to_string(), level);
        for ac in item["anchors"].as_array().unwrap() {
          let level = ac["level"].as_u64().unwrap() as usize;
          let title = ac["title"].as_str().unwrap();
          chapters.insert(title.to_string(), level);
        }
      } else if item.contains_key("level") {
        let level = item
          .get("level")
          .unwrap_or(&serde_json::Value::from(1))
          .as_u64()
          .unwrap() as usize;
        let title = item["title"].as_str().unwrap();
        chapters.insert(title.to_string(), level);
      } else {
        chapters.insert(item["title"].as_str().unwrap().to_string(), 1);
      }
    }
    chapters
  } else {
    panic!("Request failed with status: {}", res.status());
  }
}

// 获取书的详情
fn get_bookinfo(book_id: &str, cookies: &HashMap<String, String>) -> serde_json::Value {
  let url = "https://i.weread.qq.com/book/info";
  let mut params = HashMap::new();
  params.insert("bookId", book_id);

  let client = Client::new();
  let res = client
    .get(url)
    .query(&params)
    .headers(get_headers())
    .cookies(cookies)
    .send()
    .unwrap_or_else(|e| panic!("Request error: {}", e));

  if res.status().is_success() {
    res
      .json()
      .unwrap_or_else(|e| panic!("JSON parsing error: {}", e))
  } else {
    panic!("Request failed with status: {}", res.status());
  }
}

// 获取书架上所有书
fn get_bookshelf(cookies: &HashMap<String, String>) -> Vec<Book> {
  let url = "https://i.weread.qq.com/shelf/friendCommon";
  let user_vid = cookies.get("wr_vid").unwrap().to_string();
  let mut params = HashMap::new();
  params.insert("userVid", user_vid);

  let client = Client::new();
  let res = client
    .get(url)
    .query(&params)
    .headers(get_headers())
    .cookies(cookies)
    .send()
    .unwrap_or_else(|e| panic!("Request error: {}", e));

  if res.status().is_success() {
    let data: serde_json::Value = res
      .json()
      .unwrap_or_else(|e| panic!("JSON parsing error: {}", e));
    let mut books = Vec::new();
    let finish_read_books = data["finishReadBooks"]
      .as_array()
      .unwrap()
      .iter()
      .filter(|b| b.contains_key("bookId"))
      .cloned()
      .collect::<Vec<serde_json::Value>>();
    let recent_books = data["recentBooks"]
      .as_array()
      .unwrap()
      .iter()
      .filter(|b| b.contains_key("bookId"))
      .cloned()
      .collect::<Vec<serde_json::Value>>();

    for book in finish_read_books.into_iter().chain(recent_books) {
      if let Some(book_id) = book.get("bookId").and_then(|v| v.as_str()) {
        if book_id.parse::<i32>().is_ok() {
          let title = book["title"].as_str().unwrap();
          let author = book["author"].as_str().unwrap();
          let cover = book["cover"].as_str().unwrap();
          books.push(Book {
            book_id: book_id.to_string(),
            title: title.to_string(),
            author: author.to_string(),
            cover: cover.to_string(),
          });
        }
      }
    }
    books.sort_by(|a, b| a.title.cmp(&b.title));
    books
  } else {
    panic!("Request failed with status: {}", res.status());
  }
}

// 获取笔记本列表
fn get_notebooklist(cookies: &HashMap<String, String>) -> Vec<Book> {
  let url = "https://i.weread.qq.com/user/notebooks";
  let client = Client::new();
  let res = client
    .get(url)
    .headers(get_headers())
    .cookies(cookies)
    .send()
    .unwrap_or_else(|e| panic!("Request error: {}", e));

  if res.status().is_success() {
    let data: serde_json::Value = res
      .json()
      .unwrap_or_else(|e| panic!("JSON parsing error: {}", e));
    let mut books = Vec::new();
    for b in data["books"].as_array().unwrap() {
      let book = b["book"].as_object().unwrap();
      let book_id = book["bookId"].as_str().unwrap();
      let title = book["title"].as_str().unwrap();
      let author = book["author"].as_str().unwrap();
      let cover = book["cover"].as_str().unwrap();
      books.push(Book {
        book_id: book_id.to_string(),
        title: title.to_string(),
        author: author.to_string(),
        cover: cover.to_string(),
      });
    }
    books.sort_by(|a, b| a.title.cmp(&b.title));
    books
  } else {
    panic!("Request failed with status: {}", res.status());
  }
}
