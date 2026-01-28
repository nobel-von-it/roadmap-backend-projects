
const allPosts = document.getElementById("posts")

document.getElementById("new-post-form").addEventListener("submit", async (e) => {
    e.preventDefault()

    const title = document.getElementById("new-post-title").value
    const content = document.getElementById("new-post-content").value
    const category = document.getElementById("new-post-category").value
    const tags = document.getElementById("new-post-tags").value.split(", ")



    const response = await fetch("/posts", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ title, content, category, tags }),
    })

    if (response.status == 201) {
        alert("New post created")
    } else if (response.status == 400) {
        alert("Something went wrong")
        return
    }

    const data = await response.json()

    for (let [key, value] of Object.entries(data)) {
        console.log(`${key}: ${value}`)
    }

    posts.appendChild(createPost(data))
})

async function fillPosts() {
    const respnose = await fetch("/posts", {
        method: "GET",
    });

    if (respnose.status == 200) {
        alert("Loading successfull")
    } else {
        alert("Something went wrong " + respnose.status)
        return
    }

    const data = await respnose.json()
    console.log(data)
    data.forEach(post => {
        allPosts.appendChild(createPost(post))
    });
}

function createPost(dbBlogPost) {
    const postContainer = document.createElement("p")
    postContainer.classList.add("post")


    const header = document.createElement("h2")
    const postTitle = document.createElement("a")
    postTitle.classList.add("post-title")
    const postHref = "posts/" + dbBlogPost.id.toString()
    postTitle.href = postHref
    postTitle.innerText = dbBlogPost.title.toString()
    header.appendChild(postTitle)

    postContainer.appendChild(header)

    const postDate = document.createElement("div")
    postDate.classList.add("post-date")
    postDate.innerText = dbBlogPost.created_at.toString()

    postContainer.appendChild(postDate)

    const postEdit = document.createElement("button")
    postEdit.type = "button"
    postEdit.classList.add("post-edit")
    postEdit.innerText = "Edit"

    postContainer.appendChild(postEdit)

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
            alert("Post was successfully deleted")
        } else {
            alert("Something went wrong " + response.status)
        }

        // TODO: find deleted post in docuemnt and also delete it
    })

    postContainer.appendChild(postDelete)

    return postContainer
}

document.addEventListener("DOMContentLoaded", async () => {
    await fillPosts()
})
