import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { createQuest } from "../../test/mocks";
import { QuestLog } from "./QuestLog";

describe("QuestLog", () => {
  it("separates active and completed quests", () => {
    const quests = {
      q1: createQuest({ id: "q1", name: "Find Amulet", active: true, completed: false }),
      q2: createQuest({ id: "q2", name: "Slay Dragon", active: true, completed: true }),
    };
    render(<QuestLog quests={quests} />);

    expect(screen.getByText("Find Amulet")).toBeInTheDocument();
    expect(screen.getByText("Slay Dragon")).toBeInTheDocument();

    // Completed quest should have line-through
    const completed = screen.getByText("Slay Dragon");
    expect(completed.className).toContain("line-through");
  });

  it("shows active quest descriptions", () => {
    const quests = {
      q1: createQuest({
        id: "q1",
        name: "Find Amulet",
        description: "Locate the lost amulet.",
        active: true,
        completed: false,
      }),
    };
    render(<QuestLog quests={quests} />);
    expect(screen.getByText("Locate the lost amulet.")).toBeInTheDocument();
  });

  it("shows empty message when no quests", () => {
    render(<QuestLog quests={{}} />);
    expect(screen.getByText("No quests yet")).toBeInTheDocument();
  });

  it("shows empty message when all inactive", () => {
    const quests = {
      q1: createQuest({ id: "q1", active: false, completed: false }),
    };
    render(<QuestLog quests={quests} />);
    expect(screen.getByText("No quests yet")).toBeInTheDocument();
  });
});
