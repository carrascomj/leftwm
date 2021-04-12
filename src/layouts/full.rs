use crate::models::Window;
use crate::models::Workspace;
// use crate::models::WindowState;

/// Layout which gives only one window with the full desktop realestate.
/// Unlike Monocle, I goes over status bars and gaps.
pub fn update(workspace: &Workspace, windows: &mut Vec<&mut Window>) {
    let window_count = windows.len();

    if window_count == 0 {
        return;
    }

    let mut iter = windows.iter_mut();

    //maximize primary window
    {
        if let Some(monowin) = iter.next() {
            let margins = monowin.margin.clone().into_vec();
            let y_offset = (margins[0] + margins[2]) as i32;
            let x_offset = (margins[1] + margins[3]) as i32;
            monowin.set_height(workspace.full_height() + y_offset);
            monowin.set_width(workspace.full_width() + x_offset);
            monowin.set_x(workspace.full_x() - margins[0] as i32);
            monowin.set_y(workspace.full_y() - margins[1] as i32);

            monowin.set_visible(true);
        }
    }

    //hide all other windows
    {
        if window_count > 1 {
            for w in iter {
                w.set_height(workspace.height());
                w.set_width(workspace.width());
                w.set_x(workspace.x());
                w.set_y(workspace.y());

                w.set_visible(false);
            }
        }
    }
}
