import { useState } from "react";
import { crawlSeeds } from "./api";

export default function App() {
  const [seeds, setSeeds] = useState("");
  const [results, setResults] = useState([]);

  async function crawl() {
    const urls = seeds.split("\n").map(x => x.trim());
    const docs = await crawlSeeds(urls);
    setResults(docs);
  }

  return (
    <div>
      <textarea
        value={seeds}
        onChange={(e) => setSeeds(e.target.value)}
        placeholder="Enter seed URLs"
        rows={4}
      />

      <button onClick={crawl}>Crawl & Index</button>

      <div>
        {results.map(doc => (
          <div key={doc.url} style={{border: "1px solid gray", padding: "10px"}}>
            <img src={doc.favicon} width="32" height="32"/>
            <h3>{doc.title}</h3>
            <p>{doc.url}</p>

            <h4>Images</h4>
            {doc.images.map(i => <img src={i} width="100"/>)}

            <h4>Videos</h4>
            {doc.videos.map(v => <video src={v} width="200" controls/>)}

            <h4>Links</h4>
            {doc.links.map(l => <div>{l}</div>)}
          </div>
        ))}
      </div>
    </div>
  );
}
