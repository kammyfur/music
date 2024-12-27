function modalHide() {
    document.title = window.originalTitle;
    window.playerAudio.pause();
}

function crc32(input) {
    let a, o, c, n, t;

    for (o = [], c = 0; c < 256; c++) {
        a = c;

        for (let f = 0; f < 8; f++) {
            a = 1 & a ? 3988292384 ^ a >>> 1 : a >>> 1;
        }

        o[c] = a;
    }

    for (n = -1, t = 0; t < input.length; t++) {
        n = n >>> 8 ^ o[255 & (n ^ input.charCodeAt(t))];
    }

    return (-1 ^ n) >>> 0;
}

window.onhashchange = window.processHash = () => {
    modalHide();
    document.getElementById('player').classList.remove('show');
    document.getElementById('versions').classList.remove('show');

    let hash = location.hash.substring(2);
    if (location.hash !== "" && hash !== "") {
        let version;
        for (let record of window.filesProcessed) {
            for (let entry of Object.entries(record.versions)) {
                let currentVersion = entry[1];
                if (currentVersion.id === hash.split("/")[0] && entry[0] === hash.split("/")[1]) {
                    version = currentVersion;
                }
            }
        }
        if (!version) {
            location.hash = "";
            return;
        }
        window.playerTitle.innerText = document.title = version.artist + " - " + version.track +
            (version.edition.length > 0 ? " (" + version.edition.join(", ") + ")" : "") + " [" + version.year + "]";
        window.player.initialize(window.playerAudio, "https://cdn.floo.fi/watercolor/records/" + version['cdnId'] + "/stream_dash.mpd", true);
        window.playerAudio.play();
        window.playerModal.classList.add("show");
    }
}

function registerClicks(base = "js-data-list-item-") {
    Object.entries(window.filesProcessed).map((i, j) => {
        if (document.getElementById(base + j)) document.getElementById(base + j)
            .onclick = () => {
            if (i[1].versions.length < 2) {
                let version = i[1].versions[0];
                location.hash = "#/" + version.id + "/0";
            } else {
                window.versionTitle.innerText = i[1].track + (i[1].edition.length > 0 ? " (" + i[1].edition.join(", ") + ")" : "")
                window.versionList.innerHTML = i[1].versions.map((i, j) => [i, j])
                    .sort((a, b) => b[0].file.localeCompare(a[0].file))
                    .sort((a, b) => a[0].edition.length - b[0].edition.length)
                    .sort((a, b) => b[0].year - a[0].year)
                    .map(i => {
                    j = i[1];
                    i = i[0];
                    return `
                        <a style="cursor: pointer;" class="fella-list-item fella-list-link fella-list-item-padded" id="versions-item-${j}">
                            <span style="--fella-badge-notice-rgb: ${crc32(i.year.toString()).toString(16).substring(0, 6)
                                .match(/.{1,2}/g).map(i => parseInt(i, 16) + 64).join(",")} !important;"
                            class="fella-badge-notice">${i.year}</span>
                            &nbsp;&nbsp;${i.track}
                            ${i.edition.length > 0 ? i.edition.map(e => `
                                &nbsp;
                                <span style="--fella-badge-notice-rgb: ${crc32(e.toString()).toString(16).substring(0, 6)
                                    .match(/.{1,2}/g).map(i => parseInt(i, 16) + 64).join(",")} !important;"
                                class="fella-badge-notice">${e}</span>
                            `).join("") : ""}
                        </a>
                    `
                }).join("");
                window.versionModal.classList.add("show");

                i[1].versions.map((version, j) => {
                    document.getElementById("versions-item-" + j).onclick = () => {
                        location.hash = "#/" + version.id + "/" + j;
                    }
                });
            }
        };
    });
}

function search() {
    let query = document.getElementById("search").value.trim();

    if (query === "") {
        document.getElementById("js-data-list").style.display = "";
        document.getElementById("js-data-results").style.display = "none";
        document.getElementById("js-data-results").innerHTML = "";
        return;
    }

    let results = fuse.search(query).map(i => {
        i = document.getElementById("js-data-list-item-" + i['refIndex']).cloneNode(true);
        i.id = i.id.replace("-list-", "-results-");
        return i;
    });

    document.getElementById("js-data-list").style.display = "none";
    document.getElementById("js-data-results").style.display = "";
    document.getElementById("js-data-results").innerHTML = "";

    for (let item of results) {
        document.getElementById("js-data-results").insertAdjacentElement("beforeend", item);
    }

    registerClicks("js-data-results-item-");
}
