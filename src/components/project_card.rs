use leptos::*;
use crate::fetch_data::{
    github_repo_project::{
        fetch_data,
        RepoData,        
    }
};

#[component]
pub fn ProjectCard(repo_data: RepoData) -> impl IntoView {
    view!{
        <div class="max-w-md mx-auto bg-accent shadow-lg rounded-lg overflow-hidden">
            <div class="relative">
                <a href={repo_data.link.unwrap_or("#".to_string())} class="github-corner" aria-label="View source on GitHub">
                    <svg width="80" height="80" viewBox="0 0 250 250" style="fill:#151513; color:#fff; position: absolute; top: 0; border: 0; right: 0;" aria-hidden="true">
                    <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z"></path>
                    <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor" style="transform-origin: 130px 106px;" class="octo-arm"></path>
                    <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z" fill="currentColor" class="octo-body"></path>
                    </svg>
                </a>
                <div class="text-center px-6 py-4">
                    <div class="py-4 text-left text-base-100">
                        <h3 class="text-xl font-semibold text-secondary">{repo_data.name}</h3>
                        <p id="description" class="text-sm font-medium pt-3">
                            {repo_data.description.unwrap_or("---".to_string())}
                        </p>
                    </div>
                    <div class="flex justify-center py-3 text-base-100">
                        <div>
                            <p class="text-sm">Stars</p>
                            <p id="stars" class="text-lg font-semibold">
                                {repo_data.stargazers_count}
                            </p>
                        </div>
                        <div class="ml-6">
                            <p class="text-sm">Forks</p>
                            <p id="forks" class="text-lg font-semibold">
                                {repo_data.forks_count}
                            </p>
                        </div>
                        <div class="ml-6">
                            <p class="text-sm">Language</p>
                            <p id="language" class="text-lg font-semibold">
                                {repo_data.language.unwrap_or(" ".to_string())}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>



        /*
        /* From Uiverse.io by Manish-Tamang */
<aside class="bg-gray-700 text-white p-6 rounded-lg w-full max-w-lg">
  <div class="flex justify-between items-center">
    <div class="flex space-x-2 text-red-500">
      <div class="w-3 h-3 rounded-full bg-red-500"></div>
      <div class="w-3 h-3 rounded-full bg-yellow-500"></div>
      <div class="w-3 h-3 rounded-full bg-green-500"></div>
    </div>
    <p class="text-sm">github</p>
  </div>
  <div class="mt-4 text-left">
    <p class="text-green-400">$<b class="pl-2">portfolio site</b></p>
    <p class="text-white">+<b class="pl-2">next@10.2.3</b></p>
    <p class="text-white"><p class="pl-2">My personal portfolio site build with rust leptos </p></p>
    <p class="text-green-400">> <a class="pl-2" href="#">more detail </a></p>
  </div>
</aside>
*/
    }
}
