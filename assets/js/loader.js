window.playerModal = document.getElementById("player");
window.playerAudio = document.getElementById("player-el");
window.playerTitle = document.getElementById("player-title");
window.versionModal = document.getElementById("versions");
window.versionList = document.getElementById("versions-list");
window.versionTitle = document.getElementById("versions-title");
window.originalTitle = document.title;

window.onload = async () => {
    window.dom = document.createElement("div");
    window.dom.innerHTML = await (await fetch("https://music-cdn.floo.fi/")).text();
    window.files = Array.from(dom.getElementsByTagName("a")).map(i => i.href)
        .filter(i => !i.endsWith("/")).map(i => getMetadata(i));
    window.filesDeduplicated = {};

    for (let file of files) {
        if (!(file.id in window.filesDeduplicated)) window.filesDeduplicated[file.id] = {
            edition: null, artist: null, track: null, original: null, ai: null, versions: []
        }

        window.filesDeduplicated[file.id]['versions'].push(file);
    }

    for (let file of Object.values(filesDeduplicated)) {
        file.year = Math.max(...file.versions.map(i => i.year));
        file.edition = file.versions[0].edition.filter(i => !i.startsWith("v"));
        file.artist = file.versions[0].artist;
        file.track = file.versions[0].track;
        file.original = file.versions[0].original;
        file.ai = file.versions[0].ai;
    }

    document.getElementById("js-data-list").innerHTML = Object.values(window.filesDeduplicated)
        .map((i, j) => [i, j])
        .sort((a, b) => a[0].artist.localeCompare(b[0].artist))
        .sort((a, b) => b[0].year - a[0].year)
        .sort((a, b) => (b[0].ai ? -1 : 1) - (a[0].ai ? -1 : 1))
        .map(i => {
        let j = i[1];
        i = i[0];
        return `
            <a style="cursor: pointer;" id="js-data-list-item-${j}" class="fella-list-item fella-list-link fella-list-item-padded">
                ${!i.ai && !i.original ? `<span class="fella-footnotes" style="margin-top: 0;">${i.artist} - </span>` : ""}${i.track}
                ${i.edition.length > 0 ? i.edition.map(e => `
                    &nbsp;
                    <span style="--fella-badge-notice-rgb: ${crc32(e.toString()).toString(16).substring(0, 6)
                        .match(/.{1,2}/g).map(i => parseInt(i, 16) + 64).join(",")} !important;"
                    class="fella-badge-notice">${e}</span>
                `).join("") : ""}
                ${i.ai ? `
                    &nbsp;
                    <span style="--fella-badge-notice-rgb: 255,161,212 !important;" class="fella-badge-notice">AI generated</span>
                ` : ""}
                ${i.original && !i.ai ? `
                    &nbsp;
                    <span style="--fella-badge-notice-rgb: 255,132,146 !important;" class="fella-badge-notice">Original</span>
                ` : ""}
                ${i.versions.length > 1 ? `
                    &nbsp;
                    <span style="--fella-badge-notice-rgb: ${crc32(i.versions.length.toString()).toString(16).substring(0, 6)
                        .match(/.{1,2}/g).map(i => parseInt(i, 16) + 64).join(",")} !important;"
                    class="fella-badge-notice">${i.versions.length} versions</span>
                ` : ""}
            </a>
        `;
    }).join("");

    registerClicks();

    // noinspection JSUnresolvedReference
    window.fuse = new Fuse(Object.values(window.filesDeduplicated), {
        keys: [
            {name: 'artist', weight: 0.9},
            {name: 'track', weight: 1},
            {name: 'edition', weight: 0.8},
            {name: 'versions.edition', weight: 0.8},
            {name: 'versions.year', weight: 0.5},
            {name: 'versions.file',weight: 0.5},
            {name: 'versions.fullName', weight: 0.7}
        ]
    });

    document.getElementById("count").innerText = Object.keys(window.filesDeduplicated).length + " productions";
    completeLoad();
    document.getElementById("app").style.display = "";
    document.getElementById("search").value = "";
    document.getElementById("search").focus();
};
