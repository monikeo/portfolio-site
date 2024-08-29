use leptos::*;
use crate::components::title::Title;


#[derive(Debug, Clone)]
pub struct EducationInfo {
    pub title: Option<String>,
    pub year: Option<u16>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EducationInfoList {
    pub educations: Vec<EducationInfo>
}

#[component]
pub fn EducationTimeline() -> impl IntoView {
    let camtech = EducationInfo {
        title: Some("CamTech University".to_string()),
        year: Some(2022),
        description: Some("CamTech University Cyber security".to_string())
    };
    let ngs = EducationInfo {
        title: Some("Sisowath New Generation School".to_string()),
        year: Some(2015),
        description: Some("High School".to_string())
    };
    let ace = EducationInfo {
        title: Some("Australian Centre for Education".to_string()),
        year: Some(2016),
        description: Some("English Deploma".to_string())
    };
    let minsheng = EducationInfo {
        title: Some("MinSheng ZhongXue".to_string()),
        year: Some(2012),
        description: Some("Chinese Deploma Graduation".to_string())
    };
    view!{
        <ul class="timeline timeline-snap-icon max-md:timeline-compact timeline-vertical">
            <li>
                <hr />
                <div class="timeline-middle">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="h-5 w-5 text-secondary">
                            <path
                                fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                                clip-rule="evenodd" />
                    </svg>
                </div>
                <div class="timeline-start mb-10 md:text-end mx-4">
                    <time class="font-mono italic">{camtech.year.unwrap_or(0000)}</time>
                    <div class="text-lg font-black text-primary">{camtech.title.unwrap_or("UNTITLE".to_string())}</div>
                    <p>{camtech.description.unwrap_or("UNDESCRIPTION".to_string())}</p>
                </div>
                <hr />
            </li>
            <li>
                <hr />
                <div class="timeline-middle">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="h-5 w-5 text-secondary">
                            <path
                                fill-rule="evenodd"
                                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                                clip-rule="evenodd" />
                    </svg>
                </div>
                <div class="timeline-end mb-10 text-start">
                    <time class="font-mono italic">
                        {ace.year.unwrap_or(0000)}
                    </time>
                    <div class="text-lg font-black">
                        {ace.title.unwrap_or("UNTITLE".to_string())}
                    </div>
                    <p>
                        {ace.description.unwrap_or("UNDESCRIPTION".to_string())}
                    </p>
                </div>
                <hr />
            </li>

            <li>
                <hr />
                <div class="timeline-middle">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="h-5 w-5 text-secondary">
                        <path
                            fill-rule="evenodd"
                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                            clip-rule="evenodd" />
                    </svg>
                </div>
                <div class="timeline-start mb-10 md:text-end">
                    <time class="font-mono italic">
                        {ngs.year.unwrap_or(0000)}
                    </time>
                    <div class="text-lg font-black">
                        {ngs.title.unwrap_or("UNTITLE".to_string())}
                    </div>
                    <p>
                        {ngs.description.unwrap_or("UNDESCRIPTION".to_string())}
                    </p>
                </div>
                <hr />
            </li>

            <li>
                <hr />
                <div class="timeline-middle">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="h-5 w-5 text-secondary">
                        <path
                            fill-rule="evenodd"
                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                            clip-rule="evenodd" />
                    </svg>
                </div>
                <div class="timeline-end mb-10 text-start">
                    <time class="font-mono italic">
                        {minsheng.year.unwrap_or(0000)}
                    </time>
                    <div class="text-lg font-black">
                        {minsheng.title.unwrap_or("UNTITLE".to_string())}
                    </div>
                    <p>
                        {minsheng.description.unwrap_or("UNDESCRIPTION".to_string())}
                    </p>
                </div>
                <hr />
            </li>

            <li>
                <hr />
                <div class="timeline-middle">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="h-5 w-5 text-secondary">
                            <path
                                fill-rule="evenodd"
                                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                                clip-rule="evenodd" />
                    </svg>
                </div>
                <div class="timeline-start mb-10 md:text-end">
                    <time class="font-mono italic">2015</time>
                    <div class="text-lg font-black">School</div>
                    <p>

                    </p>
                </div>
            </li>
        </ul>
    }
}

#[component]
pub fn Education() -> impl IntoView {
    let title = "Education";
    view!{
        <div class="hero bg-base-100 min-h-screen">
            <div class="hero-content text-center">
                <div class="max-w-full">
                    <Title text={title}/>
                    <EducationTimeline />
                </div>
            </div>
        </div>
    }
}
