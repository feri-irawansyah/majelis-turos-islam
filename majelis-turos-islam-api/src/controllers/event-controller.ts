import { Context } from "hono";
import { getConnection } from "../middleware/connection";
import { AddEvent, AddKajian } from "../models/event-model";
import { EventService } from "../services/event-service";

// Controller get all events
export const getEventsData = async (c: Context) => {
  const pool = getConnection(c);
  const page = parseInt(c.req.query("page") || "1");
  const pageSize = parseInt(c.req.query("pageSize") || "10");
  const service = new EventService(pool, c.env.KV);
  try {
    const data = await service.getEventsData(page, pageSize);
    return c.json(data);
  } catch (err) {
    console.error(err);
    return c.json({ error: "Internal Server Error" }, 500);
  }
};

// Controller create event
export const createEvent = async (c: Context) => {
  try {
    const request = (await c.req.json()) as AddEvent;
    console.log(request);
    const pool = getConnection(c);
    const service = new EventService(pool, c.env.KV);
    await service.createEvent(request);
    return c.json({ message: "Event created successfully" });
  } catch (err: any) {
    return c.json({ error: `Error in createEvent: ${err.message}` }, 500);
  }
};

// Controller update event
export const updateEvent = async (c: Context) => {
  try {
    const request = (await c.req.json()) as AddEvent;
    const id = parseInt(c.req.param("id"));
    const pool = getConnection(c);
    const service = new EventService(pool, c.env.KV);
    await service.updateEvent(request, id);
    return c.json({ message: "Event updated successfully" });
  } catch (err: any) {
    return c.json({ error: `Error in updateEvent: ${err.message}` }, 500);
  }
};

// Controller get all kajian
export const getKajianData = async (c: Context) => {
  const pool = getConnection(c);
  const page = parseInt(c.req.query("page") || "1");
  const pageSize = parseInt(c.req.query("pageSize") || "10");
  const service = new EventService(pool, c.env.KV);
  try {
    const data = await service.getKajianData(page, pageSize);
    return c.json(data);
  } catch (err) {
    console.error(err);
    return c.json({ error: "Internal Server Error" }, 500);
  }
};

// Controller create kajian
export const createKajian = async (c: Context) => {
  try {
    const request = (await c.req.json()) as AddKajian;
    console.log(request);
    const pool = getConnection(c);
    const service = new EventService(pool, c.env.KV);
    await service.createKajian(request);
    return c.json({ message: "Event created successfully" });
  } catch (err: any) {
    return c.json({ error: `Error in createKajian: ${err.message}` }, 500);
  }
};
