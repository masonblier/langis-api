## langis-api

API server for Langis multi-language grammar and sentence database project, using actix-web 2.0 framework

##### Dependencies

- Rust / Cargo >= 1.41
- Postgresql >= 12
- Diesel command line tools with postgres feature >= 1.4 (`cargo install diesel_cli --no-default-features --features "postgres"`)

##### Install

- Create development database server

```
CREATE DATABASE langis_development;
CREATE USER langis_development WITH ENCRYPTED PASSWORD 'langis_development';
GRANT ALL ON DATABASE langis_development TO langis_development;
```

- Initialize / migrate diesel models

```
diesel migration run
```

##### Run

Runs the server. A `SECRET_KEY` environment variable of length 32 is required for password hashing, please use a unique secret key!

```
SECRET_KEY=abcdef1234567890abcdef1234567890 cargo run
```

##### License

ISC License (ISC)
Copyright 2020 Respective Contributors

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.