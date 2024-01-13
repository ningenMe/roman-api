import React, { useEffect, useState } from "react";

export const Main = (): JSX.Element => {
    const [devRootNode, setDevRootNode] = useState<chrome.bookmarks.BookmarkTreeNode>();

    useEffect(() => 
        {
            chrome.bookmarks.search({title: "dev"}, (bookmark_tree_results) => {
                chrome.bookmarks.getSubTree(bookmark_tree_results[0].id, (bookmark_details) => {
                    const root = bookmark_details[0];
                    setDevRootNode(root);
                });
            });        
        }, 
        [devRootNode]
    );

    return (
        <>
            <div>hoge</div>
            <div>{devRootNode?.title}</div>
            <div>{devRootNode?.children?.map(node => node.title)}</div>
        </>
    )
}
