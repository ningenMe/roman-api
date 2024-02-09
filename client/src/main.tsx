import React, { useEffect, useState } from "react";
import { Bookmark, BookmarkDirectory, Configuration, DefaultApi } from "./generated";

export const Main = (): JSX.Element => {
    const [devBookMarkDirectoryList, setDevBookMarkDirectoryList] = useState<Array<BookmarkDirectory>>([]);

    const config = new Configuration({
        basePath: "https://roman-api.ningenme.net"
    })
    const api = new DefaultApi(config);

    useEffect(() => 
        {
            chrome.bookmarks.search({title: "dev"}, (bookmark_tree_results) => {
                chrome.bookmarks.getSubTree(bookmark_tree_results[0].id, (bookmark_details) => {
                    const root = bookmark_details[0];
                    const bookMarkDirectoryList = root.children?.map(firstNode => {
                        const bookmarkList = firstNode.children?.map(secondNode => {
                            return {
                                title: secondNode.title,
                                url: secondNode.url ?? '',
                            }
                        }) ?? []
                        return {
                            title: firstNode.title,
                            bookmarkList: bookmarkList,
                        };
                    }) ?? [];
                    setDevBookMarkDirectoryList(bookMarkDirectoryList);
                    api.bookmarksPost({bookmarkDirectoryList: bookMarkDirectoryList})
                    .then((res) => console.log(res))
                    .catch((err) => console.log(err));
                });
            });      
        }, 
        []
    );

    return (
        <>
            <div>dev bookmark synced!</div>
        </>
    )
}
