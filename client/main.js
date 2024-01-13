chrome.bookmarks.search({title: "dev"}, (bookmark_tree_results) => {
    chrome.bookmarks.getSubTree(bookmark_tree_results[0].id, (bookmark_details) => {
        let subtree = bookmark_details[0];

        console.log(subtree);
    });
});