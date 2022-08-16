use diesel::{connection::SimpleConnection, Connection, PgConnection};

fn main() {
    let url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| String::from("postgres:///dieselbug"));

    let conn = PgConnection::establish(&url).expect("estabilish failed");

    {
        // Insert works, showing that table and triggers are fine.
        let res = conn.transaction(|| conn.batch_execute("INSERT INTO tabela (id) VALUES (1)"));
        assert!(res.is_ok());

        // Then cleaned, so it can be re-inserted later.
        let res = conn.transaction(|| conn.batch_execute("TRUNCATE tabela"));
        assert!(res.is_ok());
    }

    {
        // This fails due to trigger, but rollbacks the transaction properly
        let res = conn.transaction(|| conn.batch_execute("INSERT INTO tabela (id) VALUES (2)"));
        assert!(res.is_err());

        // So this insert works again, showing transaction is left clean and dandy.
        let res = conn.transaction(|| conn.batch_execute("INSERT INTO tabela (id) VALUES (1)"));
        assert!(res.is_ok());

        // Then cleaned, so it can be re-inserted later.
        let res = conn.transaction(|| conn.batch_execute("TRUNCATE tabela"));
        assert!(res.is_ok());
    }

    {
        // This fails due to trigger, but rollbacks the transaction properly
        let res = conn.transaction(|| conn.batch_execute("INSERT INTO tabela (id) VALUES (3)"));
        assert!(res.is_err());

        // So this insert works again, showing transaction is left clean and dandy.
        let res = conn.transaction(|| conn.batch_execute("INSERT INTO tabela (id) VALUES (1)"));
        assert!(res.is_ok());

        // Then cleaned, so it can be re-inserted later.
        let res = conn.transaction(|| conn.batch_execute("TRUNCATE tabela"));
        assert!(res.is_ok());
    }

    {
        // This fails, as it should because of the trigger
        let res = conn.transaction(|| conn.batch_execute("INSERT INTO tabela (id) VALUES (4)"));
        assert!(res.is_err());

        // This should work, since it worked before and last exec did nothing.
        let res = conn.transaction(|| conn.batch_execute("INSERT INTO tabela (id) VALUES (1)"));
        // But it doesn't, because the connection is left with open transaction.
        println!("{res:?}");
    }
}
