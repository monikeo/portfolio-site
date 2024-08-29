use leptos::*;

#[derive(Debug, Clone)]
pub struct BlogInfo {
    pub title: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub tag: Option<Vec<String>>,
    pub img_src: Option<String>,
    pub img_alt: Option<String>
}
pub struct BlogInfoList {
    pub blogs: Vec<BlogInfo>
}



#[component]
pub fn BlogCard(blog_info: BlogInfo) -> impl IntoView {
    view!{
        <div class="card bg-base-100 w-96 shadow-xl">
            <figure>
                <img
                    src={blog_info.img_src.unwrap_or("".to_string())}
                    alt="Shoes" />
            </figure>
            <div class="card-body">
                <h2 class="card-title">
                    {blog_info.title.unwrap_or("NON TITLE".to_string())}
                    <div class="badge badge-secondary">
                        {blog_info.status.unwrap_or("NON TAG".to_string())}
                    </div>
                </h2>
                <p class="text-left">
                    {blog_info.description.unwrap_or("NON DESCRIPTION".to_string())}
                </p>
                <div class="card-actions justify-end">
                    {
                        if let Some(tags) = blog_info.tag {
                            tags.iter().map(|tag| {
                                view!{
                                    <div class="badge badge-outline"> {tag} </div>
                                }
                            }).collect::<Vec<_>>()
                        } else {
                            vec![view!{
                                <div class="badge badge-outlinie"> </div>
                            }]
                        }
                    }
                    //<div class="badge badge-outline">Fashion</div>
                    //<div class="badge badge-outline">Products</div>
    </div>
  </div>
</div>
    }
}
