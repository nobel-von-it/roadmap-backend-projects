use anyhow::Result;

pub async fn render_md_to_html(md: &str) -> Result<String> {
    Ok(
        markdown::to_html_with_options(md, &markdown::Options::gfm())
            .map_err(|err| anyhow::anyhow!("Failed to render markdown to HTML: {}", err))?,
    )
}
