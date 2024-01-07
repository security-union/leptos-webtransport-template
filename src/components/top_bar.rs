use leptos::*;

use crate::components::{
    digital_ocean::DigitalOceanIcon, discord::DiscordIcon, youtube::YouTubeIcon,
};

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <div class="top-bar">
            <div class="flex space-x-2 align-middle">
                <a
                    href="https://github.com/security-union/webtransport.rs"
                    class="m-auto"
                    target="_blank"
                >
                    <img
                        src="https://img.shields.io/github/stars/security-union/webtransport.rs?style=social"
                        class="w-16"
                        alt="GitHub stars"
                    />
                </a>
                <a href="https://www.youtube.com/@SecurityUnion" class="m-auto" target="_blank">
                    <div class="w-8">
                        <YouTubeIcon/>
                    </div>
                </a>
                <a href="https://discord.gg/JP38NRe4CJ" class="m-auto" target="_blank">
                    <div class="w-8">
                        <DiscordIcon/>
                    </div>
                </a>
                <a href="https://m.do.co/c/6de4e19c5193" class="m-auto" target="_blank">
                    <div class="w-20">
                        <DigitalOceanIcon/>
                    </div>
                </a>
            </div>
            <span>
                {"Made with ❤️ by awesome developers from all over the world 🌏, hosted by Security Union 🛡️."}
            </span>
        </div>
    }
}
