import init, {CrosswordInput, generate_puz_file} from "./pkg/img2puz.js";

async function run() {
  await init("./pkg/img2puz_bg.wasm");
  document.getElementById("form")
    .addEventListener("submit", async event => {
      event.preventDefault();
      const formData = new FormData(event.target);
      const {
        across_clues, down_clues,
        title, author, copyright, notes,
        image,
      } = Object.fromEntries(formData);
      const imgBuf = await image.arrayBuffer();
      const image_array = new Uint8Array(imgBuf);
      let input = new CrosswordInput({
        across_clues, down_clues,
        title, author, copyright, notes,
        image: image_array,
      });
      let file_contents = generate_puz_file(input);
      downloadBlob(file_contents, "out.puz", "application/octet-stream");
      console.log(file_contents);
    });
}

const downloadURL = (data, fileName) => {
  const a = document.createElement('a')
  a.href = data
  a.download = fileName
  document.body.appendChild(a)
  a.style.display = 'none'
  a.click()
  a.remove()
}

const downloadBlob = (data, fileName, mimeType) => {
  const blob = new Blob([data], {
    type: mimeType
  })
  const url = window.URL.createObjectURL(blob)
  downloadURL(url, fileName)
  setTimeout(() => window.URL.revokeObjectURL(url), 1000)
}

run();