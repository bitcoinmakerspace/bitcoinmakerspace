use crate::library::{BTCMS_EMAIL, BTCMS_EST, BTCMS_GITHUB, BTCMS_NAME, BTCMS_NOSTR};
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class={classes!("flex", "flex-row", "w-full", "px-4", "max-lg:pt-20")}>
            <div class={classes!("flex", "flex-row", "w-full", "max-lg:py-8", "py-4", "justify-between", "border-t", "bordercolor")}>
                <div class={classes!("flex", "flex-row", "pl-2", "space-x-4", "items-center")}>
                    <p class={classes!("textbase", "font-op", "font-medium", "max-lg:text-xl", "text-base", "lowercase")}>
                        {format!("{}", BTCMS_NAME)}
                    </p>
                    <p class={classes!("textbase", "font-op", "font-medium", "max-lg:text-xl", "text-base", "lowercase")}>
                        {format!("{}. {}", t!("acronym.established"), BTCMS_EST)}
                    </p>
                </div>
                <div class={classes!("flex", "flex-row", "px-4", "space-x-4", "items-center", "textbase")}>
                    <a href={BTCMS_NOSTR} class={classes!("hover:opacity-80")}>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" width="40" height="40" fill="currentColor">
                            <path class="nostr-icon-background" d="m231.16,159.49c0,20.71,0,31.07-3.53,42.22-4.43,12.17-14.02,21.76-26.19,26.19-11.15,3.53-21.5,3.53-42.22,3.53h-62.46c-20.71,0-31.06,0-42.21-3.53-12.17-4.43-21.76-14.02-26.19-26.19-3.53-11.15-3.53-21.5-3.53-42.22v-62.46c0-20.71,0-31.07,3.53-42.22,4.43-12.17,14.02-21.76,26.19-26.19,11.15-3.52,21.5-3.52,42.21-3.52h62.46c20.71,0,31.07,0,42.22,3.52,12.17,4.43,21.76,14.02,26.19,26.19,3.53,11.15,3.53,21.5,3.53,42.22v62.46Z"/>
                            <path class="nostr-icon-color" d="m210.81,116.2v83.23c0,3.13-2.54,5.67-5.67,5.67h-68.04c-3.13,0-5.67-2.54-5.67-5.67v-15.5c.31-19,2.32-37.2,6.54-45.48,2.53-4.98,6.7-7.69,11.49-9.14,9.05-2.72,24.93-.86,31.67-1.18,0,0,20.36.81,20.36-10.72,0-9.28-9.1-8.55-9.1-8.55-10.03.26-17.67-.42-22.62-2.37-8.29-3.26-8.57-9.24-8.6-11.24-.41-23.1-34.47-25.87-64.48-20.14-32.81,6.24.36,53.27.36,116.05v8.38c-.06,3.08-2.55,5.57-5.65,5.57h-33.69c-3.13,0-5.67-2.54-5.67-5.67V55.49c0-3.13,2.54-5.67,5.67-5.67h31.67c3.13,0,5.67,2.54,5.67,5.67,0,4.65,5.23,7.24,9.01,4.53,11.39-8.16,26.01-12.51,42.37-12.51,36.65,0,64.36,21.36,64.36,68.69Zm-60.84-16.89c0-6.7-5.43-12.13-12.13-12.13s-12.13,5.43-12.13,12.13,5.43,12.13,12.13,12.13,12.13-5.43,12.13-12.13Z"/>
                            <rect class="nostr-icon-transparent" width="256" height="256"/>
                        </svg>
                    </a>
                    <a href={format!("mailto:{}", BTCMS_EMAIL)} class={classes!("hover:opacity-80")}>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-8 h-8">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25m19.5 0v.243a2.25 2.25 0 01-1.07 1.916l-7.5 4.615a2.25 2.25 0 01-2.36 0L3.32 8.91a2.25 2.25 0 01-1.07-1.916V6.75" />
                        </svg>
                    </a>
                    <a href={BTCMS_GITHUB} class={classes!("pl-1", "hover:opacity-80")}>
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="28" height="28" fill="currentColor">
                            <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                        </svg>
                    </a>
                </div>
            </div>
        </footer>
    }
}
