export async function crawlSeeds(seeds) {
    const res = fetch("127.0.0.1:6969/crawl", {
        method: "POST",
        headers: { "Content-Type": "application.json" },
        body: JSON.stringify(seeds),
    });
    return await res.json();
}