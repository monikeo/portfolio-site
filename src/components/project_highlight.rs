use leptos::*;

use crate::components::{
    title::Title,
    project_card::ProjectCard,
};
use crate::fetch_data::github_repo_project::RepoData;

pub fn ProjectHighLight() -> impl IntoView {
    let repo_data = RepoData {
        name: "personal site".to_string(),
        description: Some("My personal portfolio site build with rust leptos ".to_string()),
        forks_count: 0,
        language: Some("Rust".to_string()),
        created_at: "N/A".to_string(),
        stargazers_count: 0,
        link: Some("https://github.com/monikeo/portfolio-site".to_string())

    };
    view!{
        <div class="hero bg-base-100 min-h-fit">
            <div class="hero-content text-center">
                <div class="max-w-full">
                    <Title text="My Project" />
                    <ProjectCard repo_data={repo_data}/>
                </div>
            </div>
        </div>
    }
}
