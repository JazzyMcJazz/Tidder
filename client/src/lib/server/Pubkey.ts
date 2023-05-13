import { PUBLIC_API_URL } from "$env/static/public";

class Pubkey {
    pubkey: string | null = null;

    async getPubkey() {
        if (!this.pubkey) {
            const response = await fetch(PUBLIC_API_URL + "/api/pubkey");
            this.pubkey = await response.text();
        }
        return this.pubkey;
    }
}

export default new Pubkey();