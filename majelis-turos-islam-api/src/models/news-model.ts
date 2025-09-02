export type News = {
    news_id: number,
    title: string,
    content: string,
    slug: string,
    last_updated: string
}

export type AddNews = {
    title: string,
    content: string,
    slug: string
}

export type NewsArticle = {
    slug: string
    markdown: string,
}