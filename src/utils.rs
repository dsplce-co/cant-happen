use crate::REPOSITORY_NEW_ISSUE_URL;
use urlencoding::encode;

#[cfg(feature = "github")]
pub(crate) fn issue_ctor_url(why: &'static str, error: &str) -> String {
    let title = format!("Unexpected state as `{why}`");

    let body = format!(
        "<Please replace this placeholder with a brief description what command you tried to run>

```
{error}
```"
    );

    let url = REPOSITORY_NEW_ISSUE_URL.get();

    if let Some(url) = url {
        format!(
            "{url}/issues/new?title={}&body={}",
            encode(&title),
            encode(&body)
        )
    } else {
        String::from("The developer didn't provide the GitHub repository URL 😭")
    }
}

pub(crate) fn cant_happen_fmt<T, U: std::fmt::Debug>(
    why: &'static str,
    result: &Result<T, U>,
) -> String {
    let Some(error) = result.as_ref().err() else {
        return String::new();
    };

    let intro = format!(
        "\nDuring development this potential error has been marked as not likely to occur because `{why}`",
    );

    #[cfg(feature = "github")]
    {
        format!(
            "{intro}\nPlease click the link below to automatically open a GitHub issue:\n\n{}\n\nPlease describe what you were doing when it occurred. Details below will be automatically included",
            issue_ctor_url(why, &format!("{:#?}", error))
        )
    }

    #[cfg(not(feature = "github"))]
    intro
}
