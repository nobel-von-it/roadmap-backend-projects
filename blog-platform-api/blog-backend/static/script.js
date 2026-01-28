
const allPosts = document.getElementById("posts")

document.getElementById("new-post-form").addEventListener("submit", async (e) => {
    e.preventDefault()

    const title = document.getElementById("new-post-title")
    const content = document.getElementById("new-post-content")
    const category = document.getElementById("new-post-category")
    const tags = document.getElementById("new-post-tags")

    const response = await fetch("/posts", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ title: title.value, content: content.value, category: category.value, tags: tags.value.split(", ") }),
    })

    if (response.status == 201) {
        showAlert("New post created", "success")
    } else if (response.status == 400) {
        showAlert("Bad request", "error")
        return
    } else {
        showAlert("Something went wrong", "error")
        return
    }

    const data = await response.json()

    for (let [key, value] of Object.entries(data)) {
        console.log(`${key}: ${value}`)
    }

    title.value = ""
    content.value = ""
    category.value = ""
    tags.value = ""

    posts.appendChild(createPost(data))
})

function showAlert(message, type = 'success') {
    const alert = document.createElement('div');
    alert.className = `alert-message alert-${type}`;
    alert.textContent = message;
    document.body.appendChild(alert);

    setTimeout(() => {
        alert.remove();
    }, 3000);
}

async function fillPosts() {
    const respnose = await fetch("/posts", {
        method: "GET",
    });

    if (respnose.status == 200) {
        showAlert("Loading successfull", "success")
    } else {
        showAlert("Loading failed", "error")
        return
    }

    const data = await respnose.json()
    console.log(data)
    data.forEach(post => {
        allPosts.appendChild(createPost(post))
    });
}

function findPostById(id) {
    for (let i = 0; i < allPosts.children.length; i++) {
        let postId = parseInt(allPosts.children[i].id.split("-")[1])

        if (postId == id) {
            return allPosts.children[i]
        }
    }
}

function createShowPost(dbBlogPost) {
    const showContainer = document.createElement("div")
    showContainer.classList.add("show-post")

    const showTitle = document.createElement("h2")
    showTitle.innerText = dbBlogPost.title.toString()

    const showContent = document.createElement("p")
    showContent.innerText = dbBlogPost.content.toString()

    const showCategory = document.createElement("div")
    showCategory.innerText = dbBlogPost.category.toString()

    const showTags = document.createElement("div")
    showTags.innerText = dbBlogPost.tags.toString()

    const showDate = document.createElement("div")
    showDate.innerText = dbBlogPost.created_at.toString()

    showContainer.appendChild(showTitle)
    showContainer.appendChild(showContent)
    showContainer.appendChild(showCategory)
    showContainer.appendChild(showTags)
    showContainer.appendChild(showDate)

    return showContainer
}

function createEditButton(dbBlogPost) {
    const editForm = document.createElement("form")
    editForm.id = "edit-post-form"

    const editTitle = document.createElement("input")
    editTitle.type = "text"
    editTitle.name = "title"
    editTitle.value = dbBlogPost.title.toString()
    editTitle.placeholder = "some title"
    editTitle.id = "edit-post-title"

    const editContent = document.createElement("textarea")
    editContent.rows = ""
    editContent.cols = ""
    editContent.id = "edit-post-content"
    editContent.value = dbBlogPost.content.toString()

    const editCategory = document.createElement("input")
    editCategory.type = "text"
    editCategory.name = "category"
    editCategory.value = dbBlogPost.category.toString()
    editCategory.placeholder = "some category (one)"
    editCategory.id = "edit-post-category"

    const editTags = document.createElement("input")
    editTags.type = "text"
    editTags.name = "tags"
    editTags.value = dbBlogPost.tags.toString()
    editTags.placeholder = "some tags (separated by comma)"
    editTags.id = "edit-post-tags"

    const editSave = document.createElement("button")
    editSave.type = "submit"
    editSave.id = "edit-post-submit"
    editSave.innerText = "Save"
    editSave.addEventListener("click", async (e) => {
        e.preventDefault()

        const title = document.getElementById("edit-post-title").value
        const content = document.getElementById("edit-post-content").value
        const category = document.getElementById("edit-post-category").value
        const tags = document.getElementById("edit-post-tags").value.split(", ")

        const response = await fetch("/posts/" + dbBlogPost.id.toString(), {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ title, content, category, tags })
        })

        if (response.status == 200) {
            showAlert("Post updated", "success")
        } else if (response.status == 400) {
            showAlert("Bad request", "error")
            return
        } else {
            showAlert("Something went wrong", "error")
            return
        }

        const data = await response.json()

        for (let [key, value] of Object.entries(data)) {
            console.log(`${key}: ${value}`)
        }

        const post = findPostById(dbBlogPost.id)
        post.removeChild(post.children[0])
        post.appendChild(createPostInfo(data))
    })

    const editCancel = document.createElement("button")
    editCancel.type = "submit"
    editCancel.id = "edit-cancel-post-submit"
    editCancel.innerText = "Cancel"
    editCancel.addEventListener("click", (e) => {
        e.preventDefault()
        const post = findPostById(dbBlogPost.id)

        post.removeChild(post.children[0])
        post.appendChild(createPostInfo(dbBlogPost))
    })

    editForm.appendChild(editTitle)
    editForm.appendChild(editContent)
    editForm.appendChild(editCategory)
    editForm.appendChild(editTags)
    editForm.appendChild(editSave)
    editForm.appendChild(editCancel)

    return editForm
}
function createPostInfo(dbBlogPost) {
    const postInfo = document.createElement("div")

    const header = document.createElement("h2")
    const postTitle = document.createElement("button")
    postTitle.classList.add("post-title")
    postTitle.type = "button"
    postTitle.innerText = dbBlogPost.title.toString()
    postTitle.addEventListener("click", (e) => {
        e.preventDefault()
        const post = findPostById(dbBlogPost.id)
        post.removeChild(post.children[0])
        post.appendChild(createShowPost(dbBlogPost))

        const cancelShowButton = document.createElement("button")
        cancelShowButton.type = "button"
        cancelShowButton.innerText = "Cancel"
        cancelShowButton.addEventListener("click", (e) => {
            e.preventDefault()
            post.removeChild(post.children[0])
            post.appendChild(createPostInfo(dbBlogPost))
        })
        post.children[0].appendChild(cancelShowButton)
    })
    header.appendChild(postTitle)

    postInfo.appendChild(header)

    const postDate = document.createElement("div")
    postDate.classList.add("post-date")
    postDate.innerText = dbBlogPost.created_at.toString()

    postInfo.appendChild(postDate)

    const postEdit = document.createElement("button")
    postEdit.type = "button"
    postEdit.classList.add("post-edit")
    postEdit.innerText = "Edit"
    postEdit.addEventListener("click", (e) => {
        e.preventDefault()

        const post = findPostById(dbBlogPost.id)
        post.removeChild(post.children[0])
        post.appendChild(createEditButton(dbBlogPost))
    })

    postInfo.appendChild(postEdit)

    const postDelete = document.createElement("button")
    postDelete.type = "button"
    postDelete.classList.add("post-delete")
    postDelete.innerText = "Delete"
    postDelete.addEventListener("click", async (e) => {
        e.preventDefault()

        console.log("trying to delete " + postHref)
        const response = await fetch("/" + postHref, {
            method: "DELETE",
        })

        if (response.status == 204) {
            showAlert("Post was successfully deleted", "success")
        } else {
            showAlert("Something went wrong", "error")
            return
        }
        allPosts.removeChild(findPostById(dbBlogPost.id))

        // TODO: find deleted post in docuemnt and also delete it
    })

    postInfo.appendChild(postDelete)

    return postInfo
}
function createPost(dbBlogPost) {
    const postContainer = document.createElement("p")
    postContainer.id = "post-" + dbBlogPost.id
    postContainer.classList.add("post")

    postContainer.appendChild(createPostInfo(dbBlogPost))

    return postContainer
}

document.addEventListener("DOMContentLoaded", async () => {
    await fillPosts()
})
