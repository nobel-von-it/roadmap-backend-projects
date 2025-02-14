
// This is how a task should look like
//<li class="task">
//  <label class="task-checkbox-container">
//    <input type="checkbox" class="task-checkbox" />
//    <span class="checkmark"></span>
//  </label>
//  <span class="task-text">Task 1</span>
//  <button class="task-delete">x</button>
//</li>

const inputText = document.querySelector("#task-input");
const taskList = document.querySelector("#task-list");
const addTaskButton = document.querySelector("#add-task");

function createTask(text) {
	const task = document.createElement("li");
	task.classList.add("task");

	const taskLabel = document.createElement("label");
	taskLabel.classList.add("task-checkbox-container");

	const taskCheckbox = document.createElement("input");
	taskCheckbox.type = "checkbox";
	taskCheckbox.classList.add("task-checkbox");
	taskCheckbox.addEventListener("change", () => {
		if (taskCheckbox.checked) {
			task.classList.add("done");
			moveTaskDown(task);
		} else {
			task.classList.remove("done");
			moveTaskUp(task);
		}
	});
	taskLabel.appendChild(taskCheckbox);

	const taskSpan = document.createElement("span");
	taskSpan.classList.add("checkmark");
	taskLabel.appendChild(taskSpan);

	task.appendChild(taskLabel);

	const taskText = document.createElement("span");
	taskText.classList.add("task-text");
	taskText.textContent = text;
	task.appendChild(taskText);

	const taskDeleteButton = document.createElement("button");
	taskDeleteButton.classList.add("task-delete");
	taskDeleteButton.textContent = "x";
	taskDeleteButton.addEventListener("click", () => {
		taskList.removeChild(task);
	})
	task.appendChild(taskDeleteButton);

	return task;
}

function moveTaskDown(task) {
	taskList.appendChild(task);
}

function moveTaskUp(task) {
	taskList.insertBefore(task, taskList.firstChild);
}

addTaskButton.addEventListener("click", () => {
	if (inputText.value.trim() !== "") {
		const task = createTask(inputText.value);
		taskList.appendChild(task);
		inputText.value = "";
	}
})
