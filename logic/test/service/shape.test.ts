import { describe, it, expect } from "vitest";
import { ShapeService } from "../../src/form/shape/service";
import { makeInMemoryRepository } from "../support/inMemoryRepo";

describe("ShapeService", () => {
	it("creates, updates, describes, and deletes shapes with in-memory engine (no repo)", async () => {
		const svc = new ShapeService();

		const received: string[] = [];
		const tap = (k: string) => svc.on(k, (e) => received.push(e.kind));
		[
			"shape.created",
			"shape.core.set",
			"shape.state.set",
			"shape.state.patched",
			"shape.signature.set",
			"shape.signature.merged",
			"shape.facets.set",
			"shape.facets.merged",
			"shape.described",
			"shape.deleted",
		].forEach(tap);

		// create
		const id = await svc.create({ type: "system.Shape", name: "A" });
		expect(id).toBeTruthy();

		// describe baseline
		const info1 = await svc.describe(id);
		expect(info1.id).toBe(id);
		expect(info1.type).toBe("system.Shape");
		expect(info1.name).toBe("A");

		// update core
		await svc.setCore(id, { name: "A2", type: "system.Shape.Updated" });
		const info2 = await svc.describe(id);
		expect(info2.name).toBe("A2");
		expect(info2.type).toBe("system.Shape.Updated");

		// state
		await svc.setState(id, { status: "active" } as any);
		const info3 = await svc.describe(id);
		expect((info3.state as any).status).toBe("active");
		await svc.patchState(id, { meta: { ok: true } } as any);
		const info4 = await svc.describe(id);
		expect((info4.state as any).meta.ok).toBe(true);

		// signature
		await svc.setSignature(id, { a: 1 } as any);
		let info5 = await svc.describe(id);
		expect(info5.signatureKeys).toContain("a");
		await svc.mergeSignature(id, { b: 2 } as any);
		info5 = await svc.describe(id);
		expect(info5.signatureKeys).toEqual(expect.arrayContaining(["a", "b"]));
		await svc.setSignature(id, undefined);
		const infoSigCleared = await svc.describe(id);
		expect(infoSigCleared.signatureKeys).toEqual([]);

		// facets
		await svc.setFacets(id, { x: 1 } as any);
		let info6 = await svc.describe(id);
		expect(info6.facetsKeys).toContain("x");
		await svc.mergeFacets(id, { y: 2 } as any);
		info6 = await svc.describe(id);
		expect(info6.facetsKeys).toEqual(expect.arrayContaining(["x", "y"]));

			// delete
			await svc.delete(id);
			await expect(svc.get(id)).resolves.toBeUndefined();

			// event sequence sanity: first is created, last is deleted
			expect(received[0]).toBe("shape.created");
			expect(received[received.length - 1]).toBe("shape.deleted");

			// contains expected mutation events at least once
			const has = (k: string) => received.includes(k);
			[
				"shape.core.set",
				"shape.state.set",
				"shape.state.patched",
				"shape.signature.set",
				"shape.signature.merged",
				"shape.facets.set",
				"shape.facets.merged",
			].forEach((k) => expect(has(k)).toBe(true));

			// number of describes equals number of describe() calls we made
			const describedCount = received.filter((k) => k === "shape.described").length;
			expect(describedCount).toBe(9);
	});

	it("prefers repo for get() and persists changes when repo is provided", async () => {
		const repo = makeInMemoryRepository<any>();
		const svc = new ShapeService(repo as any);

		const id = await svc.create({ type: "system.Shape", name: "Repo" });

		// get via repo-backed path
		const doc1 = await svc.get(id);
		expect(doc1?.shape.core.name).toBe("Repo");

		// mutate core and verify persisted
		await svc.setCore(id, { name: "Repo2" });
		const doc2 = await svc.get(id);
		expect(doc2?.shape.core.name).toBe("Repo2");

		// convenience createAndDescribe
		const { id: id2, info } = await svc.createAndDescribe({
			type: "system.Shape",
			name: "Next",
		});
		expect(id2).toBeTruthy();
		expect(info.name).toBe("Next");

		// delete and ensure gone
		await svc.delete(id);
		await expect(svc.get(id)).resolves.toBeUndefined();
	});
});

