export function tooltip(selector, content, allowHTML) {
    tippy(selector, {
        content,
        allowHTML,
        trigger: 'click',
    });
}

export class Tooltip {
    instance;

    constructor() {
    }

    duplicate() {
        const ret = new Tooltip();
        ret.instance = this.instance;
        return ret;
    }

    create(selector, content, allowHTML) {
        const instances = tippy(selector, {
            content,
            allowHTML,
            trigger: 'click',
            hideOnClick: false,
        });
        if (instances instanceof Array) {
            this.instance = instances[0];
        }
    }

    show() {
        this.instance?.show();
    }

    hide() {
        this.instance?.hide();
    }

    setContent(content) {
        this.instance?.setContent(content);
    }

    destroy() {
        this.instance?.destroy();
    }
}
