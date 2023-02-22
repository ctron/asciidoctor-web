use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct AsciidocProperties {
    /// The Asciidoc content.
    #[prop_or_default]
    pub content: Option<String>,

    /// Options to pass on to the Asciidoc conversion process.
    #[prop_or_default]
    pub options: serde_json::Value,

    /// Classes to set on the wrapper element.
    ///
    /// Defaults to `asciidoctor`.
    #[prop_or(classes!("asciidoctor"))]
    pub class: Classes,

    /// An optional ID attribute for the wrapper element.
    #[prop_or_default]
    pub id: AttrValue,
}

/// A component showing the provided Asciidoc content, rendered to HTML.
///
/// You will still need to provide a stylesheet with the styles used by Asciidoctor.
///
/// ## Properties
///
/// Defined by [`AsciidocProperties`].
#[function_component(Asciidoc)]
pub fn asciidoc(props: &AsciidocProperties) -> Html {
    html!(
        <div id={&props.id} class={props.class.clone()}>
            {Html::from_html_unchecked(
               super::convert(props.content.as_deref().unwrap_or_default(), &props.options).unwrap_or_default().into(),
            )}
        </div>
    )
}
