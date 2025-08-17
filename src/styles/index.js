let canCopy = true;

async function loadQuote() {
    let quoteHolder = document.getElementById('loadedQuote')

    const response = await fetch('/quote')

    if (!response.ok) {
        quoteHolder.innerText = "Quote failed to load."
        return
    }

    const result = await response.json()

    quoteHolder.innerText = result.text + " - " + result.speaker
}

function copyURL(id, event) {
    if (canCopy) {
        canCopy = false;
        const copiedText = document.getElementById('copied')
        var copyText = document.getElementById(id)
        copiedText.classList.add("absolute")
        copiedText.classList.remove("hidden")

        copiedText.style.left = (event.pageX - copiedText.offsetWidth / 2) + "px";
        copiedText.style.top  = ((event.pageY - copiedText.offsetHeight / 2) - 20 ) + "px";
        copiedText.classList.add("translate-y-[-30px]")
        
        setTimeout(() => {
            copiedText.classList.remove("translate-y-[-30px]")
            copiedText.classList.add("translate-y-[0px]")
            copiedText.classList.remove("translate-y-[0px]")
            copiedText.classList.remove("absolute")
            copiedText.classList.add("hidden")
            canCopy = true;
        }, 800);
        //copyText.select()
        //copyText.setSelectionRange(0, 99999)

        navigator.clipboard.writeText(copyText.innerText)        
    }
}

document.addEventListener('DOMContentLoaded', () => {
    loadQuote()
})