"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const zod_1 = require("zod");
const orderSchema = zod_1.z.object({
    customer: zod_1.z.object({
        name: zod_1.z.string().trim().min(5),
        email: zod_1.z.string().email(),
        phone: zod_1.z.string().min(10).max(12),
    }),
    items: zod_1.z.array(zod_1.z.object({
        productId: zod_1.z.string().min(1),
        quantity: zod_1.z.number().min(1),
        price: zod_1.z.number().min(0),
    })),
    shipping: zod_1.z.enum(["standard", "express"]),
    discountCode: zod_1.z.string().optional(),
});
// Test data
const rawOrder = {
    customer: {
        name: " john Doe  ",
        email: "JOHN@EXAMPLE.COM",
        phone: "123-456-7890",
    },
    items: [
        { productId: "P1", quantity: 2, price: 10.99 },
        { productId: "P2", quantity: 1, price: 25.50 }, // Price mismatch
    ],
    shipping: "express",
    discountCode: "SAVE10",
};
const result = orderSchema.safeParse(rawOrder);
if (!result.success) {
    console.error(result.error.errors);
}
else {
    console.log(result.data);
}
