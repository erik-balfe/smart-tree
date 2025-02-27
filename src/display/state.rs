use crate::types::{DirectoryEntry, DisplayConfig};
use log::{debug, info, trace};

#[derive(Debug)]
struct DisplaySection {
    head_count: usize,
    tail_count: usize,
    total_hidden: usize,
}

pub(super) struct DisplayState<'a> {
    pub lines_remaining: usize,
    pub output: String,
    depth: usize,
    budget_stack: Vec<usize>,
    config: &'a DisplayConfig,
}

struct FormatContext {
    prefix: String,
    is_last: bool,
}

impl<'a> DisplayState<'a> {
    pub(super) fn new(max_lines: usize, config: &'a DisplayConfig) -> Self {
        info!("Initializing DisplayState with max_lines={}", max_lines);
        Self {
            lines_remaining: max_lines,
            output: String::new(),
            depth: 0,
            budget_stack: vec![max_lines],
            config,
        }
    }

    fn calculate_level_budget(&self, total_items: usize) -> usize {
        debug!(
            "calculate_level_budget: start (total={}, depth={}, remaining={})",
            total_items, self.depth, self.lines_remaining
        );

        // Early return if no lines remaining or no items
        if self.lines_remaining == 0 || total_items == 0 {
            debug!("calculate_level_budget: early return (no lines or items)");
            return 0;
        }

        // Always reserve space for directory structure
        let depth_overhead = self.depth.saturating_mul(2);
        let structure_lines = 2 + depth_overhead; // Current line + possible hidden indicator
        let available = self.lines_remaining.saturating_sub(structure_lines);

        debug!(
            "calculate_level_budget: space reservation (overhead={}, structure_lines={}, available={})",
            depth_overhead, structure_lines, available
        );

        if available == 0 {
            debug!("calculate_level_budget: no space available after reservations");
            return 0;
        }

        // Calculate base budget
        let base_budget = if self.depth == 0 {
            // Root level gets more space
            let budget = available.min(total_items);
            debug!("calculate_level_budget: root level budget = {}", budget);
            budget
        } else {
            // Nested levels get proportionally less space
            let level_factor = 3_usize.pow(self.depth as u32);
            let budget = (available / level_factor).min(total_items);
            debug!(
                "calculate_level_budget: nested level budget (factor={}, budget={})",
                level_factor, budget
            );
            budget
        };

        // Ensure we can show at least one item if possible
        let final_budget = base_budget.max(1);
        debug!("calculate_level_budget: final budget = {}", final_budget);
        final_budget
    }

    fn calculate_display_section(&self, total: usize, budget: usize) -> DisplaySection {
        debug!(
            "calculate_display_section: start (total={}, budget={}, depth={})",
            total, budget, self.depth
        );

        trace!(
            "calculate_display_section: current state (remaining={}, depth={})",
            self.lines_remaining,
            self.depth
        );

        if total <= budget {
            return DisplaySection {
                head_count: total,
                tail_count: 0,
                total_hidden: 0,
            };
        }

        // Always reserve one line for hidden items indicator
        let available = budget.saturating_sub(1);

        // For directories, show at least one item from each end if possible
        let min_head = 1;
        let min_tail = if available > 2 { 1 } else { 0 };

        // Distribute remaining space
        let remaining = available.saturating_sub(min_head + min_tail);
        let additional_head = remaining / 2;
        let additional_tail = remaining - additional_head;

        let head_count = min_head + additional_head;
        let tail_count = min_tail + additional_tail;
        let total_hidden = total.saturating_sub(head_count + tail_count);

        debug!(
            "Calculated section: head={}, tail={}, hidden={}",
            head_count, tail_count, total_hidden
        );

        DisplaySection {
            head_count,
            tail_count,
            total_hidden,
        }
    }

    fn format_entry(&self, entry: &DirectoryEntry, ctx: &FormatContext) -> String {
        trace!(
            "Formatting entry: name={}, is_dir={}, is_last={}, depth={}",
            entry.name,
            entry.is_dir,
            ctx.is_last,
            self.depth
        );

        let connector = if ctx.is_last { "└── " } else { "├── " };
        let mut output = format!("{}{}{}", ctx.prefix, connector, entry.name);

        if entry.is_gitignored && entry.is_dir {
            output.push_str(&format!(" {} [folded: system]\n", super::utils::format_metadata(entry)));
        } else {
            output.push_str(&format!(" {}\n", super::utils::format_metadata(entry)));
        }

        trace!("Formatted output: {}", output.trim());
        output
    }

    pub(super) fn show_items(&mut self, items: &[DirectoryEntry], prefix: &str) {
        info!(
            "show_items: start (count={}, depth={}, remaining={})",
            items.len(),
            self.depth,
            self.lines_remaining
        );

        trace!(
            "show_items: prefix='{}', budget_stack={:?}",
            prefix,
            self.budget_stack
        );

        if items.is_empty() || self.lines_remaining == 0 {
            debug!("Early return: empty={}, no_lines={}", items.is_empty(), self.lines_remaining == 0);
            return;
        }

        let budget = self.calculate_level_budget(items.len());
        let section = self.calculate_display_section(items.len(), budget.min(self.config.dir_limit));

        debug!(
            "Display plan: budget={}, head={}, tail={}, hidden={}",
            budget, section.head_count, section.tail_count, section.total_hidden
        );

        self.depth += 1;
        self.budget_stack.push(self.lines_remaining);

        // Show head items
        debug!("Showing head section: {} items", section.head_count);
        for (i, item) in items.iter().take(section.head_count).enumerate() {
            if self.lines_remaining == 0 {
                debug!("No lines remaining, breaking head section");
                break;
            }

            let is_last = section.tail_count == 0 && i == section.head_count - 1;
            trace!(
                "Head item {}/{}: name={}, is_last={}",
                i + 1,
                section.head_count,
                item.name,
                is_last
            );

            let ctx = FormatContext {
                prefix: prefix.to_string(),
                is_last,
            };

            let entry_line = self.format_entry(item, &ctx);
            self.output.push_str(&entry_line);
            self.lines_remaining -= 1;

            if item.is_dir && !item.is_gitignored && self.lines_remaining > 0 {
                debug!("Processing directory: {}", item.name);
                let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                self.show_items(&item.children, &new_prefix);
            }
        }

        // Show hidden items message if needed
        if section.total_hidden > 0 && self.lines_remaining > 0 {
            debug!("Adding hidden items indicator: {} items", section.total_hidden);
            self.output.push_str(&format!(
                "{}├── ... {} item{} hidden ...\n",
                prefix,
                section.total_hidden,
                if section.total_hidden == 1 { "" } else { "s" }
            ));
            self.lines_remaining -= 1;
        }

        // Show tail items if any
        if section.tail_count > 0 && self.lines_remaining > 0 {
            debug!("Showing tail section: {} items", section.tail_count);
            let tail_start = items.len() - section.tail_count;
            for (i, item) in items.iter().skip(tail_start).enumerate() {
                if self.lines_remaining == 0 {
                    debug!("No lines remaining, breaking tail section");
                    break;
                }

                let is_last = i == section.tail_count - 1;
                trace!(
                    "Tail item {}/{}: name={}, is_last={}",
                    i + 1,
                    section.tail_count,
                    item.name,
                    is_last
                );

                let ctx = FormatContext {
                    prefix: prefix.to_string(),
                    is_last,
                };

                let entry_line = self.format_entry(item, &ctx);
                self.output.push_str(&entry_line);
                self.lines_remaining -= 1;

                if item.is_dir && !item.is_gitignored && self.lines_remaining > 0 {
                    debug!("Processing directory: {}", item.name);
                    let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                    self.show_items(&item.children, &new_prefix);
                }
            }
        }

        self.depth -= 1;
        self.budget_stack.pop();
        debug!(
            "Finished level: depth={}, remaining_lines={}",
            self.depth, self.lines_remaining
        );
    }
}
