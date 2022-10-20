export const getHealth = async (_req:any, res:any) => {
    res.json({ "status": "Service alive" });
}
