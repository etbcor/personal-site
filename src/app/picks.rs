use crate::app::*;

#[component]
pub fn PicksPage() -> impl IntoView {
  let loading_hidden = create_rw_signal(false);
  let my_music_hidden = create_rw_signal(false);
  let spotify_hidden = create_rw_signal(false);
  let john_hidden = create_rw_signal(false);
  let faves_hidden = create_rw_signal(false);

  let footer_items = vec![
    ("\"Inspiration\"", loading_hidden),
    ("My Music", my_music_hidden),
    ("Playlists", spotify_hidden),
    ("Johnvertisement", john_hidden),
    ("My Favorites", faves_hidden),
  ];
  let z_idx = Some(create_rw_signal(1));

  view! {
    <LoadingWindow
      pos=WindowPos::Val((20, 20))
      size=(255, 255)
      hidden=loading_hidden
      z_idx=z_idx
      variant=LoadingWindowVariant::HomePageLink
    />
    <LinkWindow
      pos=WindowPos::Val((20, 347))
      size=(255, 255)
      hidden=my_music_hidden
      z_idx=z_idx
      id="my-music-win"
      title="My Music".to_string()
      bg_img="/assets/my-music.png"
      src="/itan"
    />
    <FavesWindow
      pos=WindowPos::Val((310, 20))
      size=(440, 582)
      hidden=faves_hidden
      z_idx=z_idx
    />
    <JohnWindow
      pos=WindowPos::Val((20, 674))
      size=(730, 90)
      hidden=john_hidden
      z_idx=z_idx
    />
    <SpotifyPlaylistWindow
      pos=WindowPos::Val((785, 20))
      size=(440, 746)
      hidden=spotify_hidden
      z_idx=z_idx
    />
    <Footer items=footer_items/>
    <GoatCounter path="/picks"/>
  }
}

#[component]
fn SpotifyPlaylistWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
  let size = create_rw_signal(size);
  let active_tab = create_rw_signal("Main");
  let content = WindowContent::Tabs((
    active_tab,
    vec![
      (
        "Main",
        view! { <div class="tab-outer" style="padding: 5px" tabindex=0>
          <SpotifyPlaylist src="1QPAKgnxEMYOBJFVmRhwM1"/>
          <SpotifyPlaylist src="0DXYn6zngiQp5AQNOToO3i"/>
          <SpotifyPlaylist src="3K8Kg0C1GVI14q3KUBqfUd"/>
          <SpotifyPlaylist src="2q5WCLRthMkrtOOApVGeYW"/>
          <SpotifyPlaylist src="0S8eDcRFe43fJHlOUAdiBE"/>
          <SpotifyPlaylist src="6LwfptFt77pViRyjBR2a3u"/>
          <SpotifyPlaylist src="37i9dQZEVXcKWDpjwB0tqt" spaced=false/>
        </div> },
      ),
      (
        "Mood",
        view! { <div class="tab-outer" style="padding: 5px" tabindex=0>
          <SpotifyPlaylist src="5JS3lDWT6W7vkghXsQHiQn"/>
          <SpotifyPlaylist src="1q7j8e6UWAC4p78QizSOqk"/>
          <SpotifyPlaylist src="6iVCPGSpMstM56Ajj0NSYI"/>
          <SpotifyPlaylist src="1TcG56ZvcjxIfs78p4U2ND"/>
          <SpotifyPlaylist src="6FQt8KArNQWlxxn5guwvFr"/>
          <SpotifyPlaylist src="0UQ9W2q0BAawJbNAuXN480"/>
          <SpotifyPlaylist src="3Qm6zeVhUSJFIyBeluWTXy"/>
          <SpotifyPlaylist src="3m5Dh6k8JzhVBHEajV86YA"/>
          <SpotifyPlaylist src="5cEz3iuf5aC9YMf3ZkI08g"/>
          <SpotifyPlaylist src="439886CxFFQD4sBKmaf2v9" spaced=false/>
        </div> },
      ),
      (
        "Genres",
        view! { <div class="tab-outer" style="padding: 5px" tabindex=0>
          <SpotifyPlaylist src="4RCXWsAR5yT7P8pfaYKQK9"/>
          <SpotifyPlaylist src="0ZarPheYW5A3Ut14uvvCYa"/>
          <SpotifyPlaylist src="1eYJLMDpgoKD0F4LUH2Ezs"/>
          <SpotifyPlaylist src="36UOLnWsxJlH7Ms5aF3exW"/>
          <SpotifyPlaylist src="2is9YFXsfFYtAYliO1Xox3"/>
          <SpotifyPlaylist src="3aLiFKFvxd4PyC3gfSIs4x"/>
          <SpotifyPlaylist src="2innGMsDBjt4m4BFWczx1P"/>
          <SpotifyPlaylist src="2LuztnBxzKkEfjvGAJx3vV"/>
          <SpotifyPlaylist src="0EujpL7Ux9PdGdVxfxXSSl"/>
          <SpotifyPlaylist src="2SxZEPs788pkeORbGs0NXj"/>
          <SpotifyPlaylist src="58cvN9oc4TnTuOKbHkgc85"/>
          <SpotifyPlaylist src="5yAQt15q8sppI3zbr1onsq"/>
          <SpotifyPlaylist src="7JLhfvA0evymAzY3TB1Opf"/>
          <SpotifyPlaylist src="1iZl1yGF0ra18Dh0jmNpjt"/>
          <SpotifyPlaylist src="7EZXrDDMBTjAtf3nWjWk5q"/>
          <SpotifyPlaylist src="5cnkxBVOu3Ompr3E7QlKa3"/>
          <SpotifyPlaylist src="37Zs98sWQAJ5SpS60hVvf1"/>
          <SpotifyPlaylist src="1fqYiy4hDIsByrWdTTYfYA"/>
          <SpotifyPlaylist src="77SM9ZJXNZtwZNlISBPz4P" spaced=false/>
        </div> },
      ),
    ],
  ));

  view! { <Window
    base=WindowBase {
      id: "spotify-win",
      title: "My Public Spotify Playlists".to_string(),
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      z_idx,
      scroll: true,
      rainbow: true,
      ..Default::default()
    }
  /> }
}

#[component]
fn SpotifyPlaylist(src: &'static str, #[prop(default = true)] spaced: bool) -> impl IntoView {
  view! {
    <iframe
      src=move || format!("https://open.spotify.com/embed/playlist/{src}?utm_source=generator")
      style="width: 400px; height: 152px; border-radius:12px"
      frameBorder="0"
      allowfullscreen=""
      allow="autoplay; clipboard-write; encrypted-media; fullscreen; picture-in-picture"
      loading="lazy"
      class:spaced=spaced
    ></iframe><br/>
  }
}

#[component]
fn FavesWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
  let size = create_rw_signal(size);
  let active_tab = create_rw_signal("Artists");
  let content = WindowContent::Tabs((
    active_tab,
    vec![
      (
        "Artists",
        view! { <div class="tab-outer" style="padding: 5px" tabindex=0>
          <p style="text-align: center">"This is my (WIP) list of favorite artists. I might change the order from time to time."</p>
        </div> },
      ),
      (
        "Albums",
        view! { <div class="tab-outer" style="padding: 5px" tabindex=0>
          <p style="text-align: center">"This is my (WIP) list of favorite albums."</p>
        </div> },
      ),
      (
        "Songs",
        view! { <div class="tab-outer" style="padding: 5px" tabindex=0>
          <p style="text-align: center">"This is my (WIP) list of favorite songs."</p>
        </div> },
      ),
    ],
  ));

  view! { <Window
    base=WindowBase {
      id: "faves-win",
      title: "My Favorite Music".to_string(),
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      z_idx,
      scroll: true,
      rainbow: true,
      ..Default::default()
    }
  /> }
}

#[component]
pub fn PicksLinkWindow(
  pos: WindowPos,
  size: (u32, u32),
  hidden: RwSignal<bool>,
  #[prop(default = None)] z_idx: Option<RwSignal<usize>>,
) -> impl IntoView {
  let size = create_rw_signal(size);
  let content = WindowContent::Page(view! { <div style="cursor: pointer">
    <video
      style="width: 100%"
      muted
      autoplay
      loop="true"
      poster="/assets/music-icon.png"
      on:mousedown=move |_| leptos_router::use_navigate()("/picks", Default::default())
      on:contextmenu=move |e| e.prevent_default()
      on:keydown=move |k| if k.key() == "Enter" { leptos_router::use_navigate()("/picks", Default::default()) }
      tabindex=0
    >
      <source src="/assets/music-icon.webm" type="video/webm"/>
    </video>
  </div> });

  view! { <Window
    base=WindowBase {
      id: "music-link-win",
      title: "My Picks".to_string(),
      content,
      pos,
      size,
      hidden,
    }
    extra=WindowExtra {
      z_idx,
      expandable: false,
      rainbow: true,
      ..Default::default()
    }
  />
  }
}
