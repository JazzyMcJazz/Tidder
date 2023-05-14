import { PUBLIC_API_URL } from "$env/static/public";
import Pubkey from "$lib/server/Pubkey";
import { redirect, type Handle } from "@sveltejs/kit";
import Jwt, { type JwtPayload }  from "jsonwebtoken";

export const handle = (async ({event, resolve}) => {

    const identity = event.cookies.get("identity");
    
    if (identity) {
        const pubkey = await Pubkey.getPubkey();
        
        const verified = Jwt.verify(identity, pubkey, { algorithms: ["RS256"], issuer: 'tidders' }) as JwtPayload;
        if (verified) {
            const { sub, username, role} = verified;
            event.locals.user = { id: sub!, username, role };
        }
    }
    
    if (!event.locals.user && event.route.id?.startsWith("/(protected)")) {
        throw redirect(302, "/");
    }

    // Set security headers
    event.setHeaders({
        // "Content-Security-Policy": `default-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' ${PUBLIC_API_URL} data:; script-src 'self' 'unsafe-inline'; connect-src 'self' ${PUBLIC_API_URL}; frame-src 'none'; form-action 'self' ${PUBLIC_API_URL}; base-uri 'none';`,
        "X-Content-Type-Options": "nosniff",
        "X-Frame-Options": "DENY",
    });

    return await resolve(event);
}) satisfies Handle;