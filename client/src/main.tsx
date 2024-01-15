import React, { useEffect, useState } from "react";
import { Configuration, DefaultApi } from "./generated";

export const Main = (): JSX.Element => {
    const [devRootNode, setDevRootNode] = useState<chrome.bookmarks.BookmarkTreeNode>();

    const config = new Configuration({
        basePath: "http://localhost"
    })
    const api = new DefaultApi(config);

    useEffect(() => 
        {
            chrome.bookmarks.search({title: "dev"}, (bookmark_tree_results) => {
                chrome.bookmarks.getSubTree(bookmark_tree_results[0].id, (bookmark_details) => {
                    const root = bookmark_details[0];
                    setDevRootNode(root);

                    api.bookmarksGet()
                    .then((res) => console.log(res))
                    .catch((err) => console.log(err));
                });
            });      
        }, 
        []
    );

    return (
        <>
            <div>hoge</div>
            <div>{devRootNode?.title}</div>
        </>
    )
}
