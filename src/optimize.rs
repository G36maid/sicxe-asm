use rayon::prelude::*;
use sicxe::frame::record::ObjectRecord;
use std::collections::VecDeque;

pub fn optimize(records: Vec<ObjectRecord>) -> String {
    let mut optimized = String::new();

    insert_headers(&mut optimized, &records);

    merge_defines(&mut optimized, &records);
    merge_refers(&mut optimized, &records);

    merge_texts(&mut optimized, &records);
    //merge_texts_parallel(&mut optimized, &records);

    insert_modifications(&mut optimized, &records);
    insert_ends(&mut optimized, &records);

    optimized
}

pub fn optimize_parallel(records: Vec<ObjectRecord>) -> String {
    let mut optimized = String::new();

    insert_headers(&mut optimized, &records);

    merge_defines(&mut optimized, &records);
    merge_refers(&mut optimized, &records);

    //merge_texts(&mut optimized, &records);
    merge_texts_parallel(&mut optimized, &records);

    insert_modifications(&mut optimized, &records);
    insert_ends(&mut optimized, &records);

    optimized
}

fn insert_headers(optimized: &mut String, records: &[ObjectRecord]) {
    records
        .iter()
        .filter_map(|r| match r {
            ObjectRecord::Header(header) => Some(header),
            _ => None,
        })
        .for_each(|header| {
            optimized.push_str(&format!(
                "H{: <6}{:06X}{:06X}\n",
                header.name, header.start, header.length
            ));
        });
}

fn merge_defines(optimized: &mut String, records: &[ObjectRecord]) {
    let mut defines = records
        .iter()
        .filter_map(|r| match r {
            ObjectRecord::Define(define) => Some(define),
            _ => None,
        })
        .collect::<Vec<_>>();
    while !defines.is_empty() {
        let line: String = defines
            .drain(..6.min(defines.len()))
            .map(|define| format!("{: <6}{:06X}", define.name, define.value))
            .collect();
        optimized.push_str(&format!("D{}\n", line));
    }
}

fn merge_refers(optimized: &mut String, records: &[ObjectRecord]) {
    let mut refers = records
        .iter()
        .filter_map(|r| match r {
            ObjectRecord::Refer(refer) => Some(refer),
            _ => None,
        })
        .collect::<Vec<_>>();
    while !refers.is_empty() {
        let line: String = refers
            .drain(..12.min(refers.len()))
            .map(|refer| format!("{: <6}", refer.name))
            .collect();
        optimized.push_str(&format!("R{}\n", line));
    }
}

fn merge_texts(optimized: &mut String, records: &[ObjectRecord]) {
    let texts = records
        .iter()
        .filter(|r| matches!(r, ObjectRecord::Text(_)))
        .collect::<Vec<_>>();
    let mut current_line = String::new();
    let mut current_start = 0;
    for r in texts {
        if let ObjectRecord::Text(r) = r {
            if current_start + current_line.len() / 2 != r.start as usize {
                if !current_line.is_empty() {
                    flush_current_line(optimized, current_start, &current_line);
                }
                current_line = String::new();
                current_start = r.start as usize;
            }

            let mut data = VecDeque::from(r.data.clone());
            while !data.is_empty() {
                let max: i32 = (60 - current_line.len() as i32) / 2;
                let wrote = max.min(data.len() as i32);

                if wrote > 0 {
                    for _ in 0..wrote {
                        if let Some(byte) = data.pop_front() {
                            current_line.push_str(&format!("{:02X}", byte));
                        }
                    }
                }

                #[cfg(debug_assertions)]
                if current_line.len() > 60 {
                    panic!("Line length exceeds 60 bytes");
                }

                if current_line.len() == 60 {
                    flush_current_line(optimized, current_start, &current_line);
                    current_line = String::new();
                    current_start += 30;
                }
            }
        }
    }

    if !current_line.is_empty() {
        flush_current_line(optimized, current_start, &current_line);
    }
}

fn merge_texts_parallel(optimized: &mut String, records: &[ObjectRecord]) {
    let texts: Vec<_> = records
        .iter()
        .filter_map(|r| match r {
            ObjectRecord::Text(r) => Some(r),
            _ => None,
        })
        .collect();

    // 按照起始地址進行排序，確保順序正確
    let mut sorted_texts = texts.clone();
    sorted_texts.sort_by_key(|r| r.start);

    // 並行處理 Text 記錄，將每段資料轉換為完整的行
    let processed_lines: Vec<_> = sorted_texts
        .par_iter()
        .map(|r| {
            let mut current_lines = Vec::new();
            let mut current_line = String::new();
            let mut current_start = r.start as usize;

            let mut data = VecDeque::from(r.data.clone());
            while !data.is_empty() {
                let max: i32 = (60 - current_line.len() as i32) / 2;
                let wrote = max.min(data.len() as i32);

                if wrote > 0 {
                    for _ in 0..wrote {
                        if let Some(byte) = data.pop_front() {
                            current_line.push_str(&format!("{:02X}", byte));
                        }
                    }
                }

                if current_line.len() == 60 {
                    current_lines.push((current_start, current_line.clone()));
                    current_line.clear();
                    current_start += 30;
                }
            }

            if !current_line.is_empty() {
                current_lines.push((current_start, current_line));
            }

            current_lines
        })
        .flatten()
        .collect();

    // 按順序合併所有行
    let mut current_line = String::new();
    let mut current_start = 0;

    for (start, line) in processed_lines {
        if current_start + current_line.len() / 2 != start {
            if !current_line.is_empty() {
                flush_current_line(optimized, current_start, &current_line);
            }
            current_line = line;
            current_start = start;
        } else {
            current_line.push_str(&line);
        }

        if current_line.len() == 60 {
            flush_current_line(optimized, current_start, &current_line);
            current_line.clear();
            current_start += 30;
        }
    }

    if !current_line.is_empty() {
        flush_current_line(optimized, current_start, &current_line);
    }
}

fn flush_current_line(optimized: &mut String, current_start: usize, current_line: &str) {
    optimized.push_str(&format!(
        "T{:06X}{:02X}{}\n",
        current_start,
        current_line.len() / 2,
        current_line
    ));
}

fn insert_modifications(optimized: &mut String, records: &[ObjectRecord]) {
    records
        .iter()
        .filter_map(|r| match r {
            ObjectRecord::Modification(modification) => Some(modification),
            _ => None,
        })
        .for_each(|modification| {
            optimized.push_str(&format!(
                "M{:06X}{:02X}{}\n",
                modification.start, modification.length, modification.symbol
            ));
        });
}

fn insert_ends(optimized: &mut String, records: &[ObjectRecord]) {
    records
        .iter()
        .filter_map(|r| match r {
            ObjectRecord::End(end) => Some(end),
            _ => None,
        })
        .for_each(|end| {
            optimized.push_str(&format!("E{:06X}\n", end.start));
        });
}
