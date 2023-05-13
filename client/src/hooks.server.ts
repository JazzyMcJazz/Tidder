import REST from "$lib/util/rest";
import { redirect, type Handle } from "@sveltejs/kit";
import Jwt, { type JwtPayload }  from "jsonwebtoken";

const pubkey = await REST.getPubkey();

export const handle = (async ({event, resolve}) => {

    const identity = event.cookies.get("identity");
    
    event.cookies.set("test", "this is a test", {
        domain: ".jazzymcjazz.dk",
        httpOnly: true,
        secure: true,
        path: "/",
        sameSite: "none",
        maxAge: 60 * 60 * 24 * 7 // 1 week
    });

    if (identity) {
        const verified = Jwt.verify(identity, pubkey, { algorithms: ["RS256"], issuer: 'tidders' }) as JwtPayload;
        if (verified) {
            const { sub, username, role} = verified;
            event.locals.user = { id: sub!, username, role };
        }
    }
    
    if (!event.locals.user && event.route.id?.startsWith("/(protected)")) {
        throw redirect(302, "/");
    }

    return await resolve(event);
}) satisfies Handle;