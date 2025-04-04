use crate::server::*;

#[test]
fn check_case_insensitivity() {
    let mut line = "jOiN #foo,&bar fubar,foobar".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: None,
            kind: CommandKind::Join {
                channels: vec!["#foo".to_string(), "&bar".to_string()],
                keys: Some(vec!["fubar".to_string(), "foobar".to_string()]),
            }
        }
    );
}

#[test]
fn parse_prefix() {
    let mut line = ":ecs.vuw.ac.nz JOIN #foo,#bar fubar,foobar".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: Some("ecs.vuw.ac.nz".to_owned()),
            kind: CommandKind::Join {
                channels: vec!["#foo".to_string(), "#bar".to_string()],
                keys: Some(vec!["fubar".to_string(), "foobar".to_string()]),
            }
        }
    );

    let mut line = ":nvx-23!nvx@ecs.vuw.ac.nz NICK dawn".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: Some("nvx-23!nvx@ecs.vuw.ac.nz".to_owned()),
            kind: CommandKind::Nick {
                nickname: "dawn".to_owned(),
            }
        }
    );

    let mut line = ":[{|21lu}]!wilkesluna@192.523.3.21 USER [{|21lu}] 0 * :dawnie".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: Some("[{|21lu}]!wilkesluna@192.523.3.21".to_owned()),
            kind: CommandKind::User {
                user_name: "[{|21lu}]".to_owned(),
                mode: 0,
                real_name: "dawnie".to_owned()
            }
        }
    );
}

#[test]
#[should_panic]
fn parse_bad_prefix() {
    // too long
    let mut line = ":lunaamethystwilkes JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();

    // digit in first location
    let mut line = ":03luna JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();

    // user without host
    let mut line = ":wilkesluna!abc JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();

    // invalid ip4addr
    let mut line = ":wilkesluna@1.1.1.1.1 JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();

    // invalid ip6addr
    let mut line = ":wilkesluna@019X JOIN #foo,#bar fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
fn parse_join() {
    let mut line = "JOIN #foo,#bar fubar,foobar".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: None,
            kind: CommandKind::Join {
                channels: vec!["#foo".to_string(), "#bar".to_string()],
                keys: Some(vec!["fubar".to_string(), "foobar".to_string()]),
            }
        }
    );

    let mut line = "JOIN 0".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: None,
            kind: CommandKind::Join {
                channels: vec!["0".to_string()],
                keys: None,
            }
        }
    );
}

#[test]
#[should_panic]
fn parse_join_no_params() {
    let mut line = "JOIN".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
#[should_panic]
fn parse_join_too_many_params() {
    let mut line = "JOIN #foo,#bar fubar,foobar foooobar".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
#[should_panic]
fn parse_join_0_more_args() {
    let mut line = "JOIN 0 fubar,foobar".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
#[should_panic]
fn parse_join_invalid_channel() {
    let mut line = "JOIN foo,#bar".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
fn parse_nick() {
    let mut line = "NICK Wiz".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: None,
            kind: CommandKind::Nick {
                nickname: "Wiz".to_owned()
            }
        }
    );

    let mut line = ":WiZ!jto@tolsun.oulu.fi NICK Kilroy".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: Some("WiZ!jto@tolsun.oulu.fi".to_owned()),
            kind: CommandKind::Nick {
                nickname: "Kilroy".to_owned()
            }
        }
    );
}

#[test]
#[should_panic]
fn parse_nick_no_nick() {
    let mut line = "NICK".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
#[should_panic]
fn parse_nick_too_long() {
    let mut line = "NICK neo-x-23-vim".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
#[should_panic]
fn parse_nick_invalid_chars() {
    let mut line = "NICK x.23".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
fn parse_user() {
    let mut line = "USER guest 0 * :Amity Blight".to_owned();
    assert_eq!(
        try_parse_from_line(&mut line).unwrap(),
        Command {
            prefix: None,
            kind: CommandKind::User {
                user_name: "guest".to_owned(),
                mode: 0,
                real_name: "Amity Blight".to_owned()
            }
        }
    );
}

#[test]
#[should_panic]
fn parse_user_bad_mode() {
    let mut line = "USER guest 11 * :Amity Blight".to_owned();
    try_parse_from_line(&mut line).unwrap();
}

#[test]
#[should_panic]
fn parse_user_no_unused() {
    let mut line = "USER guest 0 :Amity Blight".to_owned();
    try_parse_from_line(&mut line).unwrap();
}
